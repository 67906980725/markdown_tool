import sys
import requests
from bs4 import BeautifulSoup
import re
from datetime import datetime

# 移除文件名字符串中非法字符
def clean_filename(filename):
    return re.sub(r'[^\w\s-]', '', filename).strip().lower()

# 根据 url 获取处理后的标题
# 处理: 移除文件名字符串中非法字符并加上当前时间
# 防止如同一个知乎多个回答时发生文件名冲突的情况
def url_to_file_name(res):
    res.encoding = 'utf-8'
    soup = BeautifulSoup(res.text, 'html.parser')
    title = soup.title.text
    current_time = datetime.now()
    formatted_time = current_time.strftime("%Y%m%d%H%M")
    title = clean_filename(title + formatted_time)
    return title
    
def main():
    res = requests.get(sys.argv[1])
    title = url_to_file_name(res)
    print(title)

if __name__ == "__main__":
    main()
