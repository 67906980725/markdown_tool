# markdown 文件工具

安装依赖 `./install.ps1`

## url 转 markdown

如 `python ./html2md.py https://www.baidu.com`, 会在当前目录生成文件 `html2markdown.md`

## markdown 图片本地化

如

``` pwsh
cd ./md_tool_rust
cargo run "pic_down" "$home/Downloads/test.md"
```

会生成新文件 `$home/Downloads/out/test.md` 及对应的图片文件夹 `$home/Downloads/out/.asset`

## urls 转 markdown 并将图片本地化

如 `./urls_to_md.ps1 ./input.txt`. 输出到目录 `./output/out/`

## 清理未被引用的图片

如

``` pwsh
cd ./md_tool_rust
cargo run "pic_clean" "$home/Downloads/md/artical/" "$home/Downloads/md/artical/.image"
```
