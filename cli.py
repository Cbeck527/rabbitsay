# -*- coding: utf-8 -*-
import click

CONTEXT_SETTINGS = dict(help_option_names=['-h', '--help'])


@click.command(context_settings=CONTEXT_SETTINGS)
@click.option('--spacing', default=2, help='padding around message text')
@click.argument('message', metavar='"MESSAGE ON SIGN"')
def main(spacing, message):
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
    from rabbitsay import rabbitsay
    output = rabbitsay.rabbitsay(spacing, message)
    click.echo(output)
