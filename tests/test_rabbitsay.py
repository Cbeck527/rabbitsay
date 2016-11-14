import pytest

from rabbitsay import rabbitsay


@pytest.mark.parametrize("spacing", [2, 3, 4, 1000])
def test_spacing(spacing, message="testing!"):
    rabbitsay.rabbitsay(spacing, message)

# Interesting words from the Oxford Dictionary website
# https://en.oxforddictionaries.com/explore/weird-and-wonderful-words
@pytest.mark.parametrize("message", ["testing",
                                     "anguilliform",
                                     "Argus-eyed",
                                     "emmetropia"
                                     "looooooooooooooooooooooooooooooooooong word",
                                     "long and short words"])
def test_message(message, spacing=2):
    rabbitsay.rabbitsay(spacing, message)
