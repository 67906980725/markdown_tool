import sys
import requests
from bs4 import BeautifulSoup
import re


def clean_filename(filename):
    return re.sub(r'[^\w\s-]', '', filename).strip().lower()


def main():
    res = requests.get(sys.argv[1])
    res.encoding = 'utf-8'
    soup = BeautifulSoup(res.text, 'html.parser')
    title = soup.title.text
    title = clean_filename(title)
    print(title)


if __name__ == "__main__":
    main()
