## llama-cpp-2  0.1.85

git clone https://github.com/utilityai/llama-cpp-rs.git

llama.cpp dc39012cbaf8752fabecaeb60af78ccdd1dfb73b

cargo run --release --bin simple -- --prompt "The way to kill a linux process is" local F:\vol_ai\ai-models\huggingface\Qwen\Qwen2.5-0.5B-Instruct-GGUF\qwen2.5-0.5b-instruct-q4_k_m.gguf

## diffusion-rs

## whisper-rs

git clone https://github.com/tazz4843/whisper-rs.git


cargo run --example basic_use

cargo run --example audio_transcription



## stable-diffusion.cpp

commit: d9b5942d988ee36c2f2d8a2d79820e90110947c3

git clone https://github.com/leejet/stable-diffusion.cpp.git

git checkout d9b5942d988ee36c2f2d8a2d79820e90110947c3


mkdir build
cd build
cmake .. -DSD_VULKAN=ON
cmake --build . --config Release

sd -m F:\vol_ai\ai-models\huggingface\runwayml\stable-diffusion-v1-5\v1-5-pruned-emaonly.safetensors -p "a lovely cat"

## llama.cpp

commit: dc39012cbaf8752fabecaeb60af78ccdd1dfb73b	2024-11-24

commit: 64ed2091b24b2f9747148fdf49a34ed5938762c3	2024-12-2

git clone https://github.com/ggerganov/llama.cpp.git

git checkout dc39012cbaf8752fabecaeb60af78ccdd1dfb73b

llama.cpp-CMakeLists.txt

mkdir build
cd build
cmake .. -DGGML_VULKAN=ON
cmake --build . --config Release

## whisper.cpp

git clone https://github.com/ggerganov/whisper.cpp

commit:  77e3e4a09051b4d6afa64e0430492e4df7f76f1c 2024-11-25

git checkout 77e3e4a09051b4d6afa64e0430492e4df7f76f1c


## ggml

commit: 6fcbd60bc72ac3f7ad43f78c87e535f2e6206f58  2024-11-24


git clone https://github.com/ggerganov/ggml.git

git checkout 6fcbd60bc72ac3f7ad43f78c87e535f2e6206f58

