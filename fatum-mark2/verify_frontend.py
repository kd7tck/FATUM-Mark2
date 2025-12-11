from playwright.sync_api import sync_playwright

def verify_frontend():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # Navigate to the app
        page.goto("http://localhost:3000")

        # 1. Verify Title and Header
        print("Checking title...")
        assert "FATUM MARK 2" in page.title()

        # 2. Verify Tab Navigation
        print("Checking tabs...")
        page.click("button[onclick=\"showTab('profiles')\"]")
        page.wait_for_selector("#tab-profiles", state="visible")

        page.click("button[onclick=\"showTab('fengshui')\"]")
        page.wait_for_selector("#tab-fengshui", state="visible")

        # 3. Verify Feng Shui Calculation
        print("Running Feng Shui simulation...")
        # Fill form
        page.fill("#fs-year", "2024")
        page.fill("#fs-facing", "180")
        page.fill("#fs-intention", "Test Wealth")
        page.check("#fs-quantum")

        # Run
        page.click("button[onclick=\"runFengShui()\"]")

        # Wait for output
        page.wait_for_function("document.getElementById('fs-output').innerText.includes('QUANTUM FENG SHUI ANALYSIS')")

        # Take Screenshot 1: Feng Shui Dashboard
        page.screenshot(path="/home/jules/verification/feng_shui_dashboard.png")
        print("Captured Feng Shui Dashboard.")

        # 4. Verify Divination
        print("Running Divination...")
        page.click("button[onclick=\"showTab('divination')\"]")
        page.wait_for_selector("#tab-divination", state="visible")
        page.click("button[onclick=\"castHexagram()\"]")

        # Wait for hexagram result
        page.wait_for_selector("#divination-text h3")

        # Take Screenshot 2: Divination Result
        page.screenshot(path="/home/jules/verification/divination_result.png")
        print("Captured Divination Result.")

        browser.close()

if __name__ == "__main__":
    verify_frontend()
