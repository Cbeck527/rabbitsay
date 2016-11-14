"""
justsetuppythings.tumblr.com
"""

import os

from setuptools import setup, find_packages


def get_build_version():
    version_path = os.path.join(
        os.path.dirname(__file__),
        'rabbitsay',
        'VERSION')
    try:
        with open(version_path, 'r') as f:
            return int(f.read())
    except Exception:
        return 'dev0'


setup(
    name='RabbitSay',
    version='0.1.{}'.format(get_build_version()),
    packages=find_packages(),
    package_data={
        'theia': [
            'VERSION',
        ],
    },
    description='Say (possibly mean) things with a cute rabbit',
    author='Chris Becker',
    url='www.rabbitsay.wtf',
    license="MIT",
    classifiers=[
        'Development Status :: 3 - Alpha',
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Python',
        'Environment :: Console',
        'Topic :: Utilities',
    ],
    install_requires=[
        'click>=6.2',
    ],
    entry_points={
        'console_scripts': [
            'rabbitsay = cli:main',
        ],
    },
)
