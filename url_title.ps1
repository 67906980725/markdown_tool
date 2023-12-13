param([string] $url)

# 根据 url 获取网页标题
$wc = New-Object System.Net.WebClient
$wc.Encoding = [System.Text.Encoding]::UTF8
$html = $wc.DownloadString($url)
$html -match '<title.*>(.*?)</title>' | Out-Null;
$title = $matches.Count -gt 0 ? $matches[1] : ''
$title
