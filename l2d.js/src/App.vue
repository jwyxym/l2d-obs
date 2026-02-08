<template>
	<canvas
		ref = 'canvasRef'
		id = 'live2d'
	/>
</template>

<script setup lang="ts">
	import { onMounted, onUnmounted, ref } from 'vue';
	import { Config, Live2DSprite, LogLevel } from 'easy-live2d';
	import { Application, Ticker } from 'pixi.js';
	import WS, { type MSG, type MSG_DATA } from './ws';

	const canvasRef = ref<HTMLCanvasElement>();
	const app = new Application();

	Config.DebugLogEnable = false;
	Config.MouseFollow = false;
	Config.CubismLoggingLevel = LogLevel.LogLevel_Off;

	const ws = new WS();
	const live2D = {
		sprite : new Live2DSprite() as any,
		on : async (path : string) => {
			live2D.sprite.init({
				modelPath: path,
				ticker: Ticker.shared
			});
			live2D.sprite.width = canvasRef.value!.clientWidth * window.devicePixelRatio;
			live2D.sprite.height = canvasRef.value!.clientHeight * window.devicePixelRatio;
			app.stage.addChild(live2D.sprite);
		},
		drag : (x : number, y : number) => live2D.sprite._model?.setDragging(x, y),
		destroy : () => live2D.sprite.destroy()
	}

	onMounted(async () => {
		await app.init({
			view: canvasRef.value,
			backgroundAlpha: 0
		});
		ws.connect({
			onmessage : async (e : MSG) => {
				switch(e.protocol) {
					case 0:
						await live2D.on((e.data as MSG_DATA).Text)
						break;
					case 1:
						const drag = (e.data as MSG_DATA).Array;
						live2D.drag(...drag);
						break;
				}
			},
			onopen : () => ws.send({ protocol : 0, data : 0 })
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