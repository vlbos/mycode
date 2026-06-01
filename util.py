from pathlib import Path

path = Path("/Users/lisheng/Downloads/Cambridge-IELTS")
def get_audio_files():
    srt_files=[]
    for srt_file in path.glob("*.srt"):
        print(srt_file.stem)
        srt_files.append(srt_file.stem)
    srt_files=set(srt_files)
    audio_files=[]
    for audio_file in path.glob("*.wma"):
        if audio_file.stem in srt_files:
            continue
        print(audio_file.stem)
        audio_files.append(audio_file.stem)
    return audio_files
    
