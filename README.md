# LoudnessMaster

# Nam Khanh Le - nle33@uic.edu

# [Live on Vercel](https://test-loudnessmaster-sveltekit.vercel.app)

## What does your application do?

The application is an audio tool to measure loudness of user uploaded audio signals according to the [ITU-R BS.1770-4 Recommendation](https://www.itu.int/dms_pubrec/itu-r/rec/bs/R-REC-BS.1770-4-201510-I!!PDF-E.pdf), which is being widely adopted in media and broadcasting industries in recent years. LoudnessMaster does all the calculations within the browser to ensure privacy and performance of the user by using pure Rust implementation compiles down to WebAssembly. LoudnessMaster utilizes Web Workers API to ensure concurrent loudness calculation. It is mainly designed for measuring music audio files, although other audio files should work as well. Currently only support WAV files.

ADDED: Online loudness measurement by sending WAV file to the backend hosted on Render, a Rocket app running in a Docker container. The backend handles the audio processing tasks, including decoding the WAV files, deinterleaving the audio signal, and calculating the integrated loudness using an existing Rust implementation. Currently only support 16 and 32 bit audio for online measurement, since Rust does not support 24 bit integer natively.
