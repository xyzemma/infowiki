from lark import Lark, Tree, Token
import os

grammar = """
    text: textpart+
    textpart: bold | italic | code | newline | quote | std
    std: /[^_\*\`\[\]>]+/ 
    italic: "_"textpart+"_" | "*"textpart+"*"
    bold: "__"textpart+"__" | "**"textpart+"**"
    code: "`"textpart+"`"
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
    keyvalue: key "[:]" value
    key: "[!]"textpart
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
[!]title [:] xyz
[!]img [:] xyz
[!/ infobox]
"""
parser = Lark(grammar)
tree = parser.parse(source)
print(tree.pretty())