from __future__ import annotations

import subprocess
import time
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont
from playwright.sync_api import sync_playwright

FRAMES_DIR = Path("demo_frames")
OUTPUT = Path("assets/demo.mp4")
WIDTH = 1920
HEIGHT = 1080
FPS = 1
APP_URL = "http://localhost:3003"

PURPLE = (124, 58, 237)
DARK_BG = (15, 15, 20)
TEXT_LIGHT = (226, 232, 240)

SCENES = [
    {
        "title": "Lemma",
        "subtitle": (
            "The maths behind the systems you build\n"
            "For engineers and practitioners"
        ),
        "is_title_card": True,
        "duration": 4,
    },
    {
        "annotation": (
            "52 problems across probability, linear algebra, calculus,\n"
            "and discrete maths. Filter by track."
        ),
        "action": "problem_list",
        "duration": 6,
    },
    {
        "annotation": (
            "SPARK: every problem starts from a real system.\n"
            "Spam filters, gradient descent, PageRank."
        ),
        "action": "open_bayes",
        "duration": 8,
    },
    {
        "annotation": (
            "GROUND: the concept, compact.\n"
            "Definitions, intuition, one key connection."
        ),
        "action": "go_ground",
        "duration": 8,
    },
    {
        "annotation": (
            "WORK: faded worked examples.\n"
            "Watch one solved, then complete a guided exercise."
        ),
        "action": "go_work",
        "duration": 8,
    },
    {
        "annotation": (
            "SOLVE: numerical problems are auto-graded.\n"
            "Type your answer and get instant feedback."
        ),
        "action": "go_solve_numerical",
        "duration": 8,
    },
    {
        "annotation": (
            "Strategy identification: which Polya/Zeitz tactic applies?\n"
            "21 named strategies as first-class content."
        ),
        "action": "go_solve_strategy",
        "duration": 8,
    },
    {
        "annotation": (
            "Proof assessment: structure checklist, comprehension\n"
            "questions, then compare against an exemplar proof."
        ),
        "action": "go_solve_proof",
        "duration": 8,
    },
    {
        "annotation": (
            "Five-axis review: conceptual understanding, strategy,\n"
            "execution, proof quality, application awareness."
        ),
        "action": "go_review",
        "duration": 8,
    },
    {
        "annotation": (
            "Switch tracks. Linear algebra problems are connected\n"
            "to ML, graphics, and recommendation engines."
        ),
        "action": "switch_to_la",
        "duration": 6,
    },
    {
        "title": "Lemma",
        "subtitle": (
            "4 tracks / 52 problems / Foundation to Research tier\n"
            "Four-phase loop: Spark, Ground, Work, Solve\n"
            "Structured proof assessment / 21 named strategies / FSRS\n"
            "github.com/michaelmillar/lemma"
        ),
        "is_title_card": True,
        "duration": 8,
    },
]


def get_font(size: int) -> ImageFont.FreeTypeFont:
    font_paths = [
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf",
        "/usr/share/fonts/truetype/ubuntu/Ubuntu-Bold.ttf",
    ]
    for fp in font_paths:
        if Path(fp).exists():
            return ImageFont.truetype(fp, size)
    return ImageFont.load_default()


def get_font_regular(size: int) -> ImageFont.FreeTypeFont:
    font_paths = [
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
        "/usr/share/fonts/truetype/ubuntu/Ubuntu-Regular.ttf",
    ]
    for fp in font_paths:
        if Path(fp).exists():
            return ImageFont.truetype(fp, size)
    return ImageFont.load_default()


def create_title_card(title: str, subtitle: str, frame_path: Path) -> None:
    img = Image.new("RGB", (WIDTH, HEIGHT), color=DARK_BG)
    draw = ImageDraw.Draw(img)

    title_font = get_font(80)
    sub_font = get_font_regular(32)

    title_bbox = draw.textbbox((0, 0), title, font=title_font)
    title_w = title_bbox[2] - title_bbox[0]
    draw.text(
        ((WIDTH - title_w) // 2, HEIGHT // 2 - 140),
        title,
        fill=PURPLE,
        font=title_font,
    )

    for i, line in enumerate(subtitle.split("\n")):
        line_bbox = draw.textbbox((0, 0), line, font=sub_font)
        line_w = line_bbox[2] - line_bbox[0]
        draw.text(
            ((WIDTH - line_w) // 2, HEIGHT // 2 + i * 50),
            line,
            fill=TEXT_LIGHT,
            font=sub_font,
        )

    img.save(frame_path)



def add_annotation(screenshot_path: Path, annotation: str, output_path: Path) -> None:
    img = Image.open(screenshot_path)
    img = img.resize((WIDTH, HEIGHT), Image.LANCZOS)

    overlay = Image.new("RGBA", (WIDTH, HEIGHT), (0, 0, 0, 0))
    draw = ImageDraw.Draw(overlay)

    bar_height = 100
    draw.rectangle(
        [(0, HEIGHT - bar_height), (WIDTH, HEIGHT)],
        fill=(15, 15, 20, 230),
    )

    font = get_font_regular(26)
    lines = annotation.split("\n")
    y = HEIGHT - bar_height + 15
    for line in lines:
        line_bbox = draw.textbbox((0, 0), line, font=font)
        line_w = line_bbox[2] - line_bbox[0]
        draw.text(
            ((WIDTH - line_w) // 2, y),
            line,
            fill=(226, 232, 240, 255),
            font=font,
        )
        y += 36

    img = img.convert("RGBA")
    img = Image.alpha_composite(img, overlay)
    img.convert("RGB").save(output_path)


def run_demo() -> None:
    FRAMES_DIR.mkdir(exist_ok=True)

    for f in FRAMES_DIR.glob("*.png"):
        f.unlink()

    frame_num = 0

    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page(viewport={"width": WIDTH, "height": HEIGHT})

        print("Waiting for lemma server...")
        for attempt in range(30):
            try:
                page.goto(APP_URL, wait_until="networkidle", timeout=15000)
                page.wait_for_selector("main", timeout=10000)
                break
            except Exception:
                if attempt == 29:
                    raise RuntimeError("Lemma server not responding")
                time.sleep(2)

        print("Connected.")
        time.sleep(2)

        for scene in SCENES:
            label = scene.get("action", scene.get("title", "card"))
            print(f"  Frame {frame_num}: {label}")

            if scene.get("is_title_card"):
                for _ in range(scene["duration"] * FPS):
                    frame_path = FRAMES_DIR / f"frame_{frame_num:04d}.png"
                    create_title_card(scene["title"], scene["subtitle"], frame_path)
                    frame_num += 1
                continue

            action = scene.get("action", "screenshot")

            if action == "problem_list":
                page.goto(APP_URL, wait_until="networkidle", timeout=15000)
                time.sleep(1)
                pills = page.query_selector_all("button.track-pill")
                for pill in pills:
                    text = pill.text_content() or ""
                    if "probability" in text.lower():
                        pill.click()
                        time.sleep(1)
                        break

            elif action == "open_bayes":
                cards = page.query_selector_all("button.problem-card")
                for card in cards:
                    h3 = card.query_selector("h3")
                    if h3 and "Bayes" in (h3.text_content() or ""):
                        card.click()
                        break
                time.sleep(2)

            elif action == "go_ground":
                btn = page.query_selector("button.primary")
                if btn:
                    btn.click()
                    time.sleep(2)

            elif action == "go_work":
                btn = page.query_selector("button.primary")
                if btn:
                    btn.click()
                    time.sleep(2)

            elif action == "go_solve_numerical":
                btn = page.query_selector("button.primary")
                if btn:
                    btn.click()
                    time.sleep(1)
                inp = page.query_selector('input[type="text"]')
                if inp:
                    inp.fill("0.4375")
                    time.sleep(0.5)
                submit = page.query_selector("button.primary")
                if submit:
                    submit.click()
                    time.sleep(1)
                cont = page.query_selector("button.primary")
                if cont:
                    cont.click()
                    time.sleep(1)
                inp2 = page.query_selector('input[type="text"]')
                if inp2:
                    inp2.fill("0.6667")
                    time.sleep(0.5)
                sub1 = page.query_selector("button.primary")
                if sub1:
                    sub1.click()
                    time.sleep(1)

            elif action == "go_solve_strategy":
                next1 = page.query_selector("button.primary")
                if next1:
                    next1.click()
                    time.sleep(1)
                inp3 = page.query_selector('input[type="text"]')
                if inp3:
                    inp3.fill("0.0194")
                    time.sleep(0.5)
                    sub2 = page.query_selector("button.primary")
                    if sub2:
                        sub2.click()
                        time.sleep(1)
                next2 = page.query_selector("button.primary")
                if next2:
                    next2.click()
                    time.sleep(1)
                page.wait_for_selector("label.choice", timeout=5000)
                labels = page.query_selector_all("label.choice")
                for label in labels:
                    text = label.text_content() or ""
                    if "pigeonhole" in text:
                        label.click()
                        break
                time.sleep(1)

            elif action == "go_solve_proof":
                sub_btn = page.query_selector("button.primary")
                if sub_btn:
                    sub_btn.click()
                    time.sleep(1)
                next_btn = page.query_selector("button.primary")
                if next_btn:
                    next_btn.click()
                    time.sleep(1)
                checkboxes = page.query_selector_all('input[type="checkbox"]')
                for cb in checkboxes:
                    cb.check()
                    time.sleep(0.3)

            elif action == "go_review":
                proof_btn = page.query_selector("button.primary")
                if proof_btn:
                    proof_btn.click()
                    time.sleep(1)
                comp_radios = page.query_selector_all('input[type="radio"][value="true"]')
                for r in comp_radios:
                    r.check()
                    time.sleep(0.2)
                check_btn = page.query_selector("button.primary")
                if check_btn:
                    check_btn.click()
                    time.sleep(1)
                cont_btn = page.query_selector("button.primary")
                if cont_btn:
                    cont_btn.click()
                    time.sleep(1)
                ex_btn = page.query_selector("button.primary")
                if ex_btn:
                    ex_btn.click()
                    time.sleep(1)
                finish = page.query_selector("button.primary")
                if finish:
                    finish.click()
                    time.sleep(2)

            elif action == "switch_to_la":
                back_btn = page.query_selector("button.primary")
                if back_btn:
                    back_btn.click()
                    time.sleep(1)
                pills = page.query_selector_all("button.track-pill")
                for pill in pills:
                    text = pill.text_content() or ""
                    if "linear" in text.lower():
                        pill.click()
                        time.sleep(1)
                        break

            time.sleep(1)

            screenshot_path = FRAMES_DIR / f"raw_{frame_num:04d}.png"
            page.screenshot(path=str(screenshot_path), full_page=False)

            annotation = scene.get("annotation", "")
            for _ in range(scene["duration"] * FPS):
                frame_path = FRAMES_DIR / f"frame_{frame_num:04d}.png"
                if annotation:
                    add_annotation(screenshot_path, annotation, frame_path)
                else:
                    img = Image.open(screenshot_path)
                    img = img.resize((WIDTH, HEIGHT), Image.LANCZOS)
                    img.save(frame_path)
                frame_num += 1

            screenshot_path.unlink(missing_ok=True)

        browser.close()

    print(f"Generated {frame_num} frames. Encoding video...")

    OUTPUT.parent.mkdir(parents=True, exist_ok=True)

    cmd = [
        "ffmpeg", "-y",
        "-framerate", str(FPS),
        "-i", str(FRAMES_DIR / "frame_%04d.png"),
        "-c:v", "libx264",
        "-pix_fmt", "yuv420p",
        "-r", "30",
        "-preset", "medium",
        "-crf", "23",
        str(OUTPUT),
    ]
    subprocess.run(cmd, check=True)

    for f in FRAMES_DIR.glob("*.png"):
        f.unlink()
    FRAMES_DIR.rmdir()

    print(f"Done. Video saved to {OUTPUT}")


if __name__ == "__main__":
    run_demo()
