# -*- coding: utf-8 -*-
import click


CONTEXT_SETTINGS = dict(help_option_names=['-h', '--help'])

RABBIT = '''
(\__/) ||
(•ㅅ•) ||
/ 　 づ
'''

@click.command(context_settings=CONTEXT_SETTINGS)
@click.option('--spacing', default=2, help='padding around message text')
@click.argument('message', metavar='"MESSAGE ON SIGN"')
def cli(spacing, message):
    """
    \b
       ┌───────────┐
       | rabbitsay |
       └───────────┘
    (\__/) ||
    (•ㅅ•) ||
    / 　 づ

    RabbitSay - say (possibly mean) things with a cute rabbit.
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
    click.echo(sign)
