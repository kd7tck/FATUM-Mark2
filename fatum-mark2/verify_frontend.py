from playwright.sync_api import sync_playwright

def verify_decision_tree_ui():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # 1. Navigate to the page
        print("Navigating to http://localhost:3000...")
        page.goto("http://localhost:3000")

        # 2. Wait for the page to load
        page.wait_for_selector(".card")

        # 3. Check for the "Quantum Decision" card
        decision_card = page.get_by_text("Quantum Decision")
        if not decision_card.is_visible():
            print("Error: Quantum Decision card not found.")
            return

        # 4. Switch to "Decision Tree (JSON)" mode
        print("Switching to Tree mode...")
        page.get_by_label("Decision Tree (JSON)").check()

        # 5. Verify the textarea is visible
        textarea = page.locator("#decisionTreeJson")
        if not textarea.is_visible():
             print("Error: Tree JSON textarea not visible.")
             return

        # 6. Click "Load Example"
        print("Loading example tree...")
        page.get_by_text("Load Example Template").click()

        # 7. Run Simulation
        print("Clicking Initiate Simulation...")
        page.get_by_text("INITIATE SIMULATION").click()

        # 8. Wait for results
        # The result div initially says "Waiting for entropy..."
        # We wait for it to contain "Winner:"
        print("Waiting for results...")
        page.wait_for_selector("#decisionResult:has-text('Winner:')", timeout=10000)

        # 9. Take Screenshot
        print("Taking screenshot...")
        page.screenshot(path="/home/jules/verification/decision_tree_verification.png", full_page=True)
        print("Verification complete.")

        browser.close()

if __name__ == "__main__":
    verify_decision_tree_ui()
