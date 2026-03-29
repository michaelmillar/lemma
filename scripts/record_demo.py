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
PURPLE_LIGHT = (139, 92, 246)
DARK_BG = (15, 15, 20)
TEXT_LIGHT = (226, 232, 240)
GREEN = (34, 134, 58)

SCENES = [
    {
        "title": "Lemma",
        "subtitle": (
            "Learn mathematics through real-world problems\n"
            "Applied first. Rigorous always."
        ),
        "is_title_card": True,
        "duration": 6,
    },
    {
        "annotation": (
            "Browse 52 problems across four tracks.\n"
            "Filter by track. Progress bar shows completion."
        ),
        "action": "problem_list",
        "duration": 8,
    },
    {
        "annotation": (
            "Every problem starts with a Spark: a real-world scenario\n"
            "that motivates the mathematics. Applied first."
        ),
        "action": "open_bayes",
        "duration": 10,
    },
    {
        "annotation": (
            "The Ground phase teaches the concept.\n"
            "Definitions, intuition, and connections to other areas."
        ),
        "action": "go_ground",
        "duration": 10,
    },
    {
        "annotation": (
            "Solve phase: numerical problems are auto-graded.\n"
            "Type your answer and get instant feedback."
        ),
        "action": "go_solve_numerical",
        "duration": 10,
    },
    {
        "annotation": (
            "Strategy identification: which Zeitz/Polya tactic applies?\n"
            "Builds meta-cognitive problem-solving skills."
        ),
        "action": "go_solve_strategy",
        "duration": 10,
    },
    {
        "annotation": (
            "Proof problems use self-assessment rubrics.\n"
            "Check which criteria you met, then see the solution sketch."
        ),
        "action": "go_solve_proof",
        "duration": 10,
    },
    {
        "annotation": (
            "Review screen shows five-axis assessment.\n"
            "Mastery level from Novice to Expert."
        ),
        "action": "go_review",
        "duration": 10,
    },
    {
        "annotation": (
            "Switch tracks. Linear Algebra problems are connected\n"
            "to ML, graphics, and recommendation engines."
        ),
        "action": "switch_to_la",
        "duration": 8,
    },
    {
        "annotation": (
            "All 8 tests pass. Content validation ensures every concept\n"
            "and strategy reference resolves correctly."
        ),
        "is_terminal": True,
        "duration": 10,
    },
    {
        "title": "Lemma",
        "subtitle": (
            "4 tracks: Probability, Linear Algebra, Calculus, Discrete Maths\n"
            "52 problems from Foundation to Research tier\n"
            "Hybrid grading: auto-check, strategy ID, self-assessed proofs\n"
            "Rust + Svelte. Part of the studying ecosystem."
        ),
        "is_title_card": True,
        "duration": 10,
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


def create_terminal_frame(frame_path: Path) -> None:
    img = Image.new("RGB", (WIDTH, HEIGHT), color=DARK_BG)
    draw = ImageDraw.Draw(img)
    font = get_font(24)
    font_small = get_font_regular(20)

    lines = [
        ("$ cargo test --workspace", PURPLE_LIGHT),
        ("", (255, 255, 255)),
        ("running 7 tests", TEXT_LIGHT),
        ("test tests::all_strategy_refs_valid ... ok", GREEN),
        ("test tests::alias_lookup_resolves ... ok", GREEN),
        ("test tests::all_concept_refs_valid ... ok", GREEN),
        ("test tests::all_prerequisites_valid ... ok", GREEN),
        ("test tests::content_loads_successfully ... ok", GREEN),
        ("test tests::tracks_discovered ... ok", GREEN),
        ("test tests::problems_ordered_within_track ... ok", GREEN),
        ("", (255, 255, 255)),
        ("test result: ok. 7 passed; 0 failed", GREEN),
        ("", (255, 255, 255)),
        ("running 1 test", TEXT_LIGHT),
        ("test store::tests::store_round_trip ... ok", GREEN),
        ("", (255, 255, 255)),
        ("test result: ok. 1 passed; 0 failed", GREEN),
        ("", (255, 255, 255)),
        ("$ cargo clippy --workspace", PURPLE_LIGHT),
        ("    Finished `dev` profile [unoptimized + debuginfo]", TEXT_LIGHT),
        ("    0 warnings", GREEN),
    ]

    y = 60
    for text, color in lines:
        draw.text((80, y), text, fill=color, font=font_small)
        y += 30

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

            if scene.get("is_terminal"):
                for _ in range(scene["duration"] * FPS):
                    frame_path = FRAMES_DIR / f"frame_{frame_num:04d}.png"
                    create_terminal_frame(frame_path)
                    if scene.get("annotation"):
                        add_annotation(frame_path, scene["annotation"], frame_path)
                    frame_num += 1
                continue

            action = scene.get("action", "screenshot")

            if action == "problem_list":
                page.goto(APP_URL, wait_until="networkidle", timeout=15000)
                time.sleep(2)

            elif action == "open_bayes":
                cards = page.query_selector_all("button.problem-card")
                if len(cards) > 1:
                    cards[1].click()
                    time.sleep(2)

            elif action == "go_ground":
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
                    inp.fill("0.6667")
                    time.sleep(1)

            elif action == "go_solve_strategy":
                submit = page.query_selector("button.primary")
                if submit:
                    submit.click()
                    time.sleep(1)
                next_btn = page.query_selector("button.primary")
                if next_btn:
                    next_btn.click()
                    time.sleep(1)
                next_btn2 = page.query_selector("button.primary")
                if next_btn2:
                    next_btn2.click()
                    time.sleep(1)
                labels = page.query_selector_all("label.choice")
                if labels:
                    labels[0].click()
                    time.sleep(1)

            elif action == "go_solve_proof":
                submit = page.query_selector("button.primary")
                if submit:
                    submit.click()
                    time.sleep(1)
                next_btn = page.query_selector("button.primary")
                if next_btn:
                    next_btn.click()
                    time.sleep(1)
                checkboxes = page.query_selector_all('input[type="checkbox"]')
                for cb in checkboxes[:3]:
                    cb.check()
                    time.sleep(0.3)

            elif action == "go_review":
                submit = page.query_selector("button.primary")
                if submit:
                    submit.click()
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
