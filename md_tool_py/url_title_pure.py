# ai 翻译 未测试
import urllib.request  
from bs4 import BeautifulSoup  

# 根据 url 获取网面标题
def get_title(url):  
    with urllib.request.urlopen(url) as url:  
        html = url.read().decode('utf-8')  
  
    soup = BeautifulSoup(html, 'html.parser')  
    title = soup.title.string  
  
    return title  
  
url = input("请输入网址: ")  
print(get_title(url))