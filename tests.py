# api 请求示例
import os
import requests
import util
# 请求地址
url = "http://127.0.0.1:9977/api"
# 请求参数  file:音视频文件，language：语言代码，model：模型，response_format:text|json|srt
# 返回 code==0 成功，其他失败，msg==成功为ok，其他失败原因，data=识别后返回文字
file_path="/Users/lisheng/Downloads/Cambridge-IELTS/"
audio_files=util.get_audio_files()
for name in audio_files:
    file_name=file_path+name
    files = {"file": open(file_name+".wma", "rb")}
    data={ 'model': 'tiny.en', 'language': 'en', 'data_type': 'srt'}
    response = requests.request("POST", url, timeout=600, data=data,files=files)
    print(response.json())
    res=response.json()['data']
    srt_file_path=file_name+".srt"
    with open(srt_file_path, "w", encoding="utf-8") as f:
        f.write(res)
'''
response
{'code': 0, 'data': [{'end_time': '00:00:16,000', 'line': 1, 'start_time': '00:00:00,000', 'text': '在后面的做,本期我们介绍电磁罚的公园里'}, {'end_
time': '00:00:19,000', 'line': 2, 'start_time': '00:00:16,000', 'text': '首先我们拿到的是一款电磁罚'}, {'end_time': '00:00:25,000', 'line': 3, 'sta
rt_time': '00:00:19,000', 'text': '这上面有三个孔,这里有两个孔'}, {'end_time': '00:00:32,000', 'line': 4, 'start_time': '00:00:25,000', 'text': '这
里有土,带看一下,A、B,下面是RPS带看下'}], 'msg': 'ok'}
'''

