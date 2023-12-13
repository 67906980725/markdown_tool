# ai 翻译 未测试
import re  
import urllib.request  
import os  
from arc import AtomicU32  
import requests  
import pathlib  

  
def mkdir(path):  
    if not os.path.exists(path):  
        os.makedirs(path)
        
def down(url: str, file_path: str) -> None:  
    try:  
        response = requests.get(url)  
        with open(file_path, 'wb') as file:  
            response.content.copy_to(file)  
    except Exception as e:  
        print(f"Error occurred: {e}")
          
def trans_line_pics_to_local(line, parent_path_name, md_name, cnt):  
    re_pattern = r"!\[.*?]\((.*?)\)"  
    result = re.sub(re_pattern, lambda capture: trans_line_one_pic_to_local(capture, parent_path_name, md_name, cnt.clone()), line)  
    return result  
  
def trans_line_one_pic_to_local(capture, parent_path_name, md_name, cnt):  
    url = capture.group(1) if capture.group(1) else ""  
    uri = urllib.parse.urlparse(url)  
    pic_origin_name = uri.path.split("/")[-1] if uri.path else ""  
    pic_suffix = os.path.splitext(pic_origin_name)[1] if pic_origin_name else ""  
      
    n = cnt.fetch_add(1)  
    uuid = "{:0>3}".format(n)  
    f_name_new = f"{uuid}{pic_suffix}"  
      
    asset_dir_name = ".asset"  
    parent_path = os.path.join(parent_path_name, asset_dir_name, md_name)  
    os.makedirs(parent_path, exist_ok=True)  
      
    file_path = os.path.join(parent_path, f_name_new)  
    urllib.request.urlretrieve(url, file_path)  
      
    url_new = f"./{asset_dir_name}/{md_name}/{f_name_new}"  
    return f"![]({url_new})"
  
def trans_md_pics_to_local(md_path):  
    # println!("find md local image: {}", &md_path);  
    origin_file_name = os.path.basename(md_path)  
    new_file_name = origin_file_name.replace(" ", "")  
    file_name = new_file_name  
    file_name_less_suffix = os.path.splitext(file_name)[0]  
  
    with open(md_path, "r") as file:  
        buf_reader = BufReader(file)  
  
        parent_path = os.path.dirname(md_path)  
        parent_path_name = parent_path  
        out_path = os.path.join(parent_path, "out")  
        mkdir(out_path)  
  
        new_file_path = os.path.join(out_path, file_name)  
        with open(new_file_path, "w") as new_file:  
            writer = LineWriter(BufferedWriter(new_file))  
  
            cnt = AtomicU32(0)  
            for line in buf_reader:  
                line = line.strip()  
                if line:  
                    new_line = trans_line_pics_to_local(line, out_path, file_name_less_suffix, cnt.clone())  
                    writer.write(new_line + "\n").unwrap()  
            writer.flush().unwrap()  
        
def main(args):  
    if len(args) < 2:  
        raise ValueError("需要 1 个参数 by MD_PATHS")  
    for arg in args[1].split(","):  
        trans_md_pics_to_local(arg.strip())  