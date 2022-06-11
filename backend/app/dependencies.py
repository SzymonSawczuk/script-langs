from pathlib import Path
from fastapi.templating import Jinja2Templates

def get_templates():
    BASE_DIR = Path(".").resolve().parent
    return Jinja2Templates(directory=Path(BASE_DIR, "frontend/static"))
