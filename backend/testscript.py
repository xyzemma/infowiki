import requests
headers = {'Content-type': 'application/json'}
print(requests.post("http://0.0.0.0:3000/user",json={"name":"test","age":12},headers=headers).text)