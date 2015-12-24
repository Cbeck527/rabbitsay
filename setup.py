from setuptools import setup


setup(
    name='RabbitSay',
    version='0.1',
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
    py_modules=['rabbitsay'],
    install_requires=[
        'Click',
    ],
    entry_points='''
        [console_scripts]
        rabbitsay=rabbitsay:cli
    ''',
)
