importScripts('./loudness_measurement.js');

let wasmReady = false;
let wasmMemory;

let wasm;

const initWasm = async () => {
	wasm = await wasm_bindgen('./loudness_measurement_bg.wasm');
	postMessage('WASM loaded');
	wasmReady = true;
};

initWasm();

self.onmessage = function (event) {
	if (wasmReady && event.data.action === 'processAudio') {
		let buffer = event.data.buffer;
		let sampleRate = event.data.sampleRate;
		let res = wasm_bindgen.process(buffer, sampleRate);
		if (res instanceof Error) {
			postMessage({
				type: 'error',
				value: res
			});
		} else {
			postMessage({
				type: 'loudnessResult',
				value: res
			});
		}
	}
};
