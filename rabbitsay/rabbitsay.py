# -*- coding: utf-8 -*-

RABBIT = '''
(\__/) ||
(•ㅅ•) ||
/ 　 づ
'''


def rabbitsay(spacing, message):
    """
       ┌───────────┐
       | rabbitsay |
       └───────────┘
    (\__/) ||
    (•ㅅ•) ||
    / 　 づ

    Function to generate rabbit and sign with custom content
    """
    lines = message.split()
    width = max(map(len, lines)) + spacing
    edge = '─' * width
    innards = '\n'.join(
        '   |{:^{width}}|'.format(line, width=width)
        for line in lines
    )
    sign = '   ┌{edge}┐\n{innards}\n   └{edge}┘{rabbit}'.format(
        innards=innards,
        edge=edge,
        rabbit=RABBIT,
    )
    return sign
