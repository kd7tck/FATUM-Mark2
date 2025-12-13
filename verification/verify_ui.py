import time
from playwright.sync_api import sync_playwright

def verify_ui():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        try:
            page.goto("http://127.0.0.1:3000")

            # 1. Verify Navigation Groups
            print("Checking Navigation...")
            page.wait_for_selector(".nav-group", state="visible")
            page.screenshot(path="verification/1_nav_restructure.png")

            # 2. Verify Tooltip
            print("Checking Tooltips...")
            # Hover over a button to trigger tooltip
            btn = page.locator("button[data-tooltip='View Past Analysis Records']")
            btn.hover()
            # Wait for tooltip box to be visible
            tooltip = page.locator("#tooltip-box")
            # Wait for it to become visible
            tooltip.wait_for(state="visible", timeout=2000)
            page.screenshot(path="verification/2_tooltip.png")

            # 3. Verify Feng Shui Sidebar Details
            print("Checking Sidebar...")
            # Click on 'Quantum Configuration' summary to expand it
            summary = page.locator("summary", has_text="QUANTUM CONFIGURATION")
            summary.click()
            time.sleep(0.5) # Animation
            page.screenshot(path="verification/3_sidebar_details.png")

            print("Verification Complete.")

        except Exception as e:
            print(f"Error: {e}")
        finally:
            browser.close()

if __name__ == "__main__":
    verify_ui()
