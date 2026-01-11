# Under most circumstances maturin generates this automatically
# However since `kkdlib` folder exists it treats this as a rust/python mixed project
from .kkdlib import *

__doc__ = kkdlib.__doc__
if hasattr(kkdlib, "__all__"):
    __all__ = kkdlib.__all__