import requests
headers = {'Content-type': 'application/json'}
print(requests.post("http://0.0.0.0:3000/createpage",json={"name":"test","text":'[# phead]__Hello__ _is a word._ [br]`print("hello world");`[#/phead][! infobox][title [:] xyz][img [:] xyz][!/ infobox]'},headers=headers).text)
print(requests.get("http://0.0.0.0:3000/wiki/test"))
try:
    requests.post("http://0.0.0.0:3000/debugquit")
except:
    quit()