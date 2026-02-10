<template>
	<canvas
		ref = 'canvasRef'
		id = 'live2d'
		:style = "{
			transform : `scaleY(${[1, 2].includes(live2D.position) ? -1 : 1})`
		}"
	/>
</template>

<script setup lang="ts">
	import { onMounted, onUnmounted, reactive, ref } from 'vue';
	import { Config, Live2DSprite, LogLevel } from 'easy-live2d';
	import { Application, Ticker } from 'pixi.js';
	import WS, { type MSG_DATA } from './ws';

	const canvasRef = ref<HTMLCanvasElement>();
	const app = new Application();

	Config.DebugLogEnable = false;
	Config.MouseFollow = false;
	Config.CubismLoggingLevel = LogLevel.LogLevel_Off;

	const ws = new WS();
	const live2D = reactive({
		sprite : new Live2DSprite() as any,
		position : 0,
		on : async (path : string) => {
			live2D.sprite.init({
				modelPath: 'vts/' + path,
				ticker: Ticker.shared
			});
			live2D.sprite.width = canvasRef.value!.clientWidth * window.devicePixelRatio;
			live2D.sprite.height = canvasRef.value!.clientHeight * window.devicePixelRatio;
			app.stage.addChild(live2D.sprite);
			while(!live2D.sprite._model)
				await new Promise(resolve => setTimeout(resolve, 100));
			Object.defineProperty(live2D.sprite._model, '_lipsync', {
				value: true,
				writable: true,
				enumerable: true,
				configurable: true
			});
			Object.defineProperty(live2D.sprite._model._wavFileHandler, 'update', {
				value: () => {},
				writable: true,
				enumerable: true,
				configurable: true
			});
		},
		openMouth : (value : number) => {
			Object.defineProperty(live2D.sprite._model._wavFileHandler, '_lastRms', {
				value: value,
				writable: true,
				enumerable: true,
				configurable: true
			});
		},
		drag : (x : number, y : number) => live2D.sprite._model?.setDragging(x, y),
		destroy : () => live2D.sprite.destroy()
	});

	onMounted(async () => {
		await app.init({
			view: canvasRef.value,
			backgroundAlpha: 0
		});
		ws.connect({
			onmessage : async (protocol : number, data : MSG_DATA) => {
				switch(protocol) {
					case 0:
						live2D.position = data.Number;
						break;
					case 1:
						await live2D.on(data.Text);
						break;
					case 2:
						live2D.drag(...data.Array);
						break;
					case 3:
						live2D.openMouth(data.Number);
						break;
				}
			},
			onopen : () => {
				ws.send({ protocol : 0, data : 0 });
				if (!live2D.sprite._model)
					ws.send({ protocol : 1, data : 0 });
			}
		})
	});

	onUnmounted(() => {
		live2D.destroy();
		ws.disconnect();
	});

</script>

<style>
	canvas {
		position: absolute;
		top: 0%;
		right: 0%;
		width: 100%;
		height: 100%;
	}
</style>