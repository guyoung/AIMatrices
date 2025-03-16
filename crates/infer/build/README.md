## llama-cpp-2  0.1.85

git clone https://github.com/utilityai/llama-cpp-rs.git

llama.cpp dc39012cbaf8752fabecaeb60af78ccdd1dfb73b

cargo run --release --bin simple -- --prompt "The way to kill a linux process is" local F:\vol_ai\ai-models\huggingface\Qwen\Qwen2.5-0.5B-Instruct-GGUF\qwen2.5-0.5b-instruct-q4_k_m.gguf

## diffusion-rs

## whisper-rs

commit: 10c6501bd05a697e014f1bee3a84e5664290c489 2025-3-9

git clone https://github.com/tazz4843/whisper-rs.git


cargo run --example basic_use

cargo run --example audio_transcription



## stable-diffusion.cpp

commit: 10c6501bd05a697e014f1bee3a84e5664290c489 2025-3-9

git clone https://github.com/leejet/stable-diffusion.cpp.git

git checkout 10c6501bd05a697e014f1bee3a84e5664290c489


mkdir build
cd build
cmake .. -DSD_VULKAN=ON
cmake --build . --config Release

sd -m F:\vol_ai\ai-models\huggingface\runwayml\stable-diffusion-v1-5\v1-5-pruned-emaonly.safetensors -p "a lovely cat"

## llama.cpp

commit: 06c2b1561d8b882bc018554591f8c35eb04ad30e	2025-3-1

git clone https://github.com/ggerganov/llama.cpp.git

git checkout 06c2b1561d8b882bc018554591f8c35eb04ad30e

llama.cpp-CMakeLists.txt

mkdir build
cd build
cmake .. -DGGML_VULKAN=ON
cmake --build . --config Release

## whisper.cpp

commit:  c98681e6d5f4b51fd49bf9868cc9fc27b48097ed 2025-02-28

git clone https://github.com/ggerganov/whisper.cpp



git checkout c98681e6d5f4b51fd49bf9868cc9fc27b48097ed


## ggml

commit: ff9052988b76e137bcf92bb335733933ca196ac0  2025-02-28


git clone https://github.com/ggerganov/ggml.git

git checkout ff9052988b76e137bcf92bb335733933ca196ac0

