from lark import Lark, Tree, Token
import os

grammar = """
    text: textpart+
    textpart: bold | italic | code | newline | quote | std
    std: /[^_\*\`\[\]>]+/ 
    italic: "_"textpart+"_" | "*"textpart+"*"
    bold: "__"textpart+"__" | "**"textpart+"**"
    code: "`" codetext "`"
    codetext: /[^`]+/
    quote: ">"textpart+"<"
    newline: "[br]"
    start: section+
    section: box | pagesection
    pagesection: psstart text* psend
    psstart: "[# "CNAME"]" | "[#"CNAME"]"
    psend: "[#/ "CNAME"]" | "[#/"CNAME"]"
    box: boxstart boxcontent boxend
    boxstart: "[!" CNAME "]" | "[! " CNAME "]"
    boxcontent: keyvalue+
    keyvalue: "[" key "[:]" value "]"
    key: textpart
    value: textpart
    boxend: "[!/ " "]" | "[!/" CNAME "]"

    %import common.ESCAPED_STRING
    %import common.CNAME
    %import common.NUMBER
    %import common.WS
    %ignore WS
    """
source = """
[# phead]
__Hello__ is a word. [br]
`print("hello world");`
[#/phead]
[! infobox]
[title [:] xyz]
[img [:] xyz]
[!/ infobox]
"""


def traverse_tree(node):
    if isinstance(node, Tree):
        global htmltext
        htmltext = f"""<!DOCTYPE html>
<head title="{pagetitle}"
<body>
<h1>{pagetitle}<h1>
<hr><br>"""

        print(f"Tree: {node.data}")

        for child in node.children:
            traverse_tree(child)

    elif isinstance(node, Token):
        print(f"Token: {node.type} -> {node.value}")

def parsefile(source_code):
    tree = parser.parse(source_code)
    traverse_tree(tree)

pagetitle = "test"
parser = Lark(grammar)
def parse(source,title):
    tree = parser.parse(source)
    pagetitle = title
    traverse_tree(tree)
parse(source,pagetitle)
htmltext = htmltext + "\n</body>"
print(htmltext)