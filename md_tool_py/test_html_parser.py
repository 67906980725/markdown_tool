from html.parser import HTMLParser



class MyHTMLParser(HTMLParser):
    def handle_starttag(self, tag, attrs):
        print("Encountered a start tag:", tag)
    def handle_endtag(self, tag):
        print("Encountered an end tag :", tag)
    def handle_data(self, data):
        print("Encountered some data  :", data)
parser = MyHTMLParser()
parser.feed('<title>Test</title>'
            '<h1>Parse me!</h1><a href="baidu.com"><div>百度</div></a>')

# todo fix
# ](website.com)
# 登录/注册topic](website.com/topic/num) normal...
# [name](website.com/user)​](website.com/question/num)
