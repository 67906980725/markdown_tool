# ai 翻译 未测试
import os  
import subprocess
import url_title
  
def process_url(url):  
    # title = subprocess.check_output(["./md_tool_py/url_title.py", url]).decode('utf-8').strip()  
    title = url_title.url_to_file_name(url)
    new_file_path = os.path.join("./output", f"{title}.md")  
    subprocess.run(["python", "./md_tool_py/html2md.py", url])  
    os.rename("./html2markdown.md", new_file_path)  
    new_file_full_path = os.path.abspath(new_file_path)  
  
    os.chdir("./md_tool_rust")  
    subprocess.run(["cargo", "run", "pic_down", new_file_full_path])  
    os.chdir("..")  
  
def read_multi_line_text():  
    lines = []  
    while True:  
        line = input()  
        if line == "":  
            break  
        lines.append(line)  
    return "\n".join(lines)  
  
file = "./input.txt"  
if os.path.exists(file):  
    with open(file, 'r') as f:  
        lines = f.readlines()  
    if len(lines) > 0:  
        for line in lines:  
            if line.strip() == "" or line.startswith("#"):  
                continue  
            url = line.strip()  
            process_url(url)  
else:  
    print("Enter the urls you want to download")  
    while True:  
        line = input()  
        if line.strip() == "" or line.startswith("#"):  
            break  
        process_url(line)  
  
os.chdir("./output")  
for filename in os.listdir("."):  
    if filename.endswith(".md"):  
        os.remove(filename)  
os.chdir("..")