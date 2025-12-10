from playwright.sync_api import sync_playwright

def verify_visual_editor():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # Navigate to the app
        page.goto("http://localhost:3000")

        # Click on Visual Editor Mode
        page.click("input[value='visual']")

        # Verify the container is visible
        editor_container = page.locator("#dVisualInput")
        assert editor_container.is_visible()

        # Add a new node
        page.click("button:has-text('+ Add Node')")

        # Verify new node card appears (should be 2 nodes now: start + new)
        nodes = page.locator(".node-card")
        assert nodes.count() >= 2

        # Take screenshot
        page.screenshot(path="verification/visual_editor.png", full_page=True)

        browser.close()

if __name__ == "__main__":
    verify_visual_editor()
