# - map
# - command parser
# - character + inventory

# <DIRECTIONS NORTH EAST WEST SOUTH NE NW SE SW UP DOWN IN OUT LAND>



class Token():
    def __init__(self, name, value, line):
        self.name = name
        self.value = value
        self.line = line
        self.parent = None
        self.escaped = False
        
    def describe(self, offset=""):
        print(offset + self.name + ", " + self.value)

class Node():
    def __init__(self, name):
        self.name = name
        self.children = []
        self.parent = None
        self.escaped = False

    def add(self, *args):
        for a in args:
            a.parent = self
            self.children.append(a)

    def prepend(self, arg):
        arg.parent = self
        self.children.insert(0, arg)

    def describe(self, offset=""):
        print(offset + self.name)
        offset += "  "
        for n in self.children:
            n.describe(offset)

token_names = {"\\": "ESCAPE",
               "\"": "QUOTE",
               "'": "SINGLE",
               "<": "LEFT_ARROW",
               ">": "RIGHT_ARROW",
               "(": "LEFT_PAREN",
               ")": "RIGHT_PAREN",
               " ": "SPACE",
               "\t": "TAB",
               ";": "COMMENT",
               ".": "PERIOD",
               ",": "COMMA",
               "?": "QUESTION",
               "%": "PERCENT"}

def tokenize(f):
    out = []
    line_count = 1
    while True:
        line = f.readline()
        if not line:
            break
        line = line.strip()
        word = ""
        while len(line) > 0:
            if line[0] in token_names:
                if len(word) > 0:
                    out.append(Token("WORD", word, line_count))
                out.append(Token(token_names[line[0]], line[0], line_count))
                word = ""
            else:
                word = word + line[0]
            line = line[1:]
        if len(word) > 0:
            out.append(Token("WORD", word, line_count))
        out.append(Token("NL", "\n", line_count))
        line_count += 1
    return out

def parse(tokens):
    i = 0
    matched_rule = False
    safety_iter = 0
    safety_lim = 100
    while len(tokens) > 1:
        if i >= len(tokens):
            i = 0
            matched_rule = False
        elif matched_rule:
            i = i - 3 if (i - 3 >= 0) else 0
        while i < len(tokens):
            safety_iter += 1
            t0 = tokens[i].name
            t1 = tokens[i+1].name if (i+1) < len(tokens) else None
            # ESCAPE + [any basic token] -> WORD
            if t0 == "ESCAPE" and (t1 in token_names.values() or t1 == "WORD" or t1 == "NL"):
                tmp = Token("WORD", tokens[i+1].value, tokens[i+1].line)
                del tokens[i+1]
                tokens[i] = tmp
                matched_rule = True
                break
            # TAB -> SPACE
            elif t0 == "TAB":
                tokens[i].name = "SPACE"
                matched_rule = True
                break
            # SPACE + SPACE -> SPACE
            elif t0 == "SPACE" and t1 == "SPACE":
                del tokens[i]
                matched_rule = True
                break
            # SPACE + NL -> NL
            elif t0 == "SPACE" and t1 == "SPACE":
                del tokens[i]
                matched_rule = True
                break
            # NL + SPACE -> NL
            elif t0 == "SPACE" and t1 == "SPACE":
                del tokens[i+1]
                matched_rule = True
                break
            # NL + NL -> NL
            elif t0 == "SPACE" and t1 == "SPACE":
                del tokens[i]
                matched_rule = True
                break
            # [SINGLE, COMMA, PERIOD, PERCENT] -> access_modifier
            elif t0 == "SINGLE" or t0 == "COMMA" or t0 == "PERIOD" or t0 == "PERCENT":
                tmp = Node("access_modifier")
                tmp.add(tokens[i])
                tokens[i] = tmp
                matched_rule = True
                break
            # access_modifier + [SINGLE, COMMA, PERIOD, PERCENT, SPACE] -> access_modifier
            elif t0 == "access_modifier" and (t1 == "SINGLE" or t1 == "COMMA" or t1 == "PERIOD" or t1 == "PERCENT" or t1 == "SPACE"):
                tokens[i].add(tokens[i+1])
                del tokens[i+1]
                matched_rule = True
                break
            # WORD -> part_word
            elif t0 == "WORD":
                tmp = Node("part_word")
                tmp.add(tokens[i])
                tokens[i] = tmp
                matched_rule = True
                break
            # access_modifier + WORD -> part_word
            elif t0 == "access_modifier" and t1 == "WORD":
                tmp = Node("part_word")
                tmp.add(tokens[i], tokens[i+1])
                del tokens[i+1]
                tokens[i] = tmp
                matched_rule = True
                break
            # access_modifier + [pointy_func, smooth_func] -> [pointy_func, smooth_func]
            elif t0 == "access_modifier" and (t1 == "pointy_func" or t1 == "smooth_func"):
                tokens[i+1].prepend(tokens[i])
                del tokens[i]
                matched_rule = True
                break 
            # part_word + [QUESTION, WORD] -> part_word
            elif t0 == "part_word" and (t1 == "QUESTION" or t1 == "WORD"):
                tokens[i].add(tokens[i+1])
                del tokens[i+1]
                matched_rule = True
                break
            # part_word + [not ESCAPE] -> full_word + [not ESCAPE]
            elif t0 == "part_word" and not t1 == "ESCAPE":
                tokens[i].name = "full_word"
                matched_rule = True
                break
            # COMMENT -> comment_builder
            elif t0 == "COMMENT":
                tmp = Node("comment_builder")
                tmp.add(tokens[i])
                tokens[i] = tmp
                matched_rule = True
                break
            # QUOTE -> quote_builder
            elif t0 == "QUOTE":
                tmp = Node("quote_builder")
                tmp.add(tokens[i])
                tokens[i] = tmp
                matched_rule = True
                break
            # LEFT_ARROW -> pointy_builder
            elif t0 == "LEFT_ARROW":
                tmp = Node("pointy_builder")
                tmp.add(tokens[i])
                tokens[i] = tmp
                matched_rule = True
                break
            # LEFT_PAREN -> smooth_builder
            elif t0 == "LEFT_PAREN":
                tmp = Node("smooth_builder")
                tmp.add(tokens[i])
                tokens[i] = tmp
                matched_rule = True
                break
            # comment_builder + SPACE -> comment_builder
            elif t0 == "comment_builder" and t1 == "SPACE":
                tokens[i].add(tokens[i+1])
                del tokens[i+1]
                matched_rule = True
                break
            # comment_builder + [full_word, full_comment, full_quote, pointy_func, smooth_func, NL, or None] -> full_comment
            elif t0 == "comment_builder" and (t1 == "full_word" or t1 == "full_comment" or t1 == "full_quote" or t1 == "pointy_func" or t1 == "smooth_func" or t1 == "NL" or t1 == None):
                tokens[i].name = "full_comment"
                tokens[i].add(tokens[i+1])
                del tokens[i+1]
                matched_rule = True
                break
            # quote_builder + QUOTE -> full_quote
            elif t0 == "quote_builder" and t1 == "QUOTE":
                tokens[i].name = "full_quote"
                tokens[i].add(tokens[i+1])
                del tokens[i+1]
                matched_rule = True
                break
            # quote_builder + [not ESCAPE] -> quote_builder
            elif t0 == "quote_builder" and not t1 == "ESCAPE":
                tokens[i].add(tokens[i+1])
                del tokens[i+1]
                matched_rule = True
                break
            # pointy_builder + RIGHT_ARROW -> pointy_func
            elif t0 == "pointy_builder" and t1 == "RIGHT_ARROW":
                tokens[i].name = "pointy_func"
                tokens[i].add(tokens[i+1])
                del tokens[i+1]
                matched_rule = True
                break
            # pointy_builder + [SPACE, NL, full_word, full_comment, full_quote, pointy_func, smooth_func] -> pointy_builder
            elif t0 == "pointy_builder" and (t1 == "SPACE" or t1 == "NL" or t1 == "full_word" or t1 == "full_comment" or t1 == "full_quote" or t1 == "pointy_func" or t1 == "smooth_func"):
                tokens[i].add(tokens[i+1])
                del tokens[i+1]
                matched_rule = True
                break
            # smooth_builder + RIGHT_PAREN -> smooth_func
            elif t0 == "smooth_builder" and t1 == "RIGHT_PAREN":
                tokens[i].name = "smooth_func"
                tokens[i].add(tokens[i+1])
                del tokens[i+1]
                matched_rule = True
                break
            # smooth_builder + [SPACE, NL, full_word, full_comment, full_quote, pointy_func, smooth_func] -> smooth_builder
            elif t0 == "smooth_builder" and (t1 == "SPACE" or t1 == "NL" or t1 == "full_word" or t1 == "full_comment" or t1 == "full_quote" or t1 == "pointy_func" or t1 == "smooth_func"):
                tokens[i].add(tokens[i+1])
                del tokens[i+1]
                matched_rule = True
                break
            # funky_bunch + [SPACE, NL, full_word, full_comment, full_quote, pointy_func, smooth_func] -> funky_bunch
            elif t0 == "funky_bunch" and (t1 == "SPACE" or t1 == "NL" or t1 == "full_word" or t1 == "full_comment" or t1 == "full_quote" or t1 == "pointy_func" or t1 == "smooth_func"):
                tokens[i].add(tokens[i+1])
                del tokens[i+1]
                matched_rule = True
                break
            # [SPACE, NL, full_word, full_comment, full_quote, pointy_func, smooth_func] + [SPACE, NL, full_word, full_comment, full_quote, pointy_func, smooth_func] -> funky_bunch
            elif (t0 == "SPACE" or t0 == "NL" or t0 == "full_word" or t0 == "full_comment" or t0 == "full_quote" or t0 == "pointy_func" or t0 == "smooth_func") and (t1 == "SPACE" or t1 == "NL" or t1 == "full_word" or t1 == "full_comment" or t1 == "full_quote" or t1 == "pointy_func" or t1 == "smooth_func"):
                tmp = Node("funky_bunch")
                tmp.add(tokens[i], tokens[i+1])
                del tokens[i+1]
                tokens[i] = tmp
                matched_rule = True
                break
            i += 1
        if not matched_rule:
            print("could not match rule")
            #for t in tokens:
            #    print(t.describe())
            quit()
    return tokens

keywords = []

with open("edited-zork/test.txt", "r") as f:
    tokens = tokenize(f)
    #for t in tokens:
    #    print(t.name)
    parse(tokens)
    #tokens[0].describe()
