#!/usr/bin/env python3
import os
import sys
import json
import random
import subprocess

# Define slide content matching what's supported in the Rust implementation
SLIDE_CONTENT = {
    "hero": [
        {"headline": "Introducing AI Analytics", "subheadline": "Transform your data into insights", "badge": "NEW"},
        {"headline": "The Future of Work", "subheadline": "AI-powered productivity tools", "badge": "LAUNCH"},
    ],
    "cta": [
        {"headline": "Ready to Get Started?", "button_text": "Start Free Trial", "button_url": "#", "subtext": "No credit card required"},
        {"headline": "Join 10,000+ Teams", "button_text": "See Pricing", "button_url": "#", "subtext": "Instant activation"},
    ],
    "feature": [
        {"icon": "🚀", "title": "Lightning Fast", "description": "10x faster processing", "number": "10x"},
        {"icon": "🔒", "title": "Enterprise Security", "description": "Bank-grade encryption", "number": "256bit"},
    ],
    "list": [
        {"title": "Key Benefits", "items": [{"title": "Automated workflows", "description": "Save 5+ hours weekly"}, {"title": "Real-time insights", "description": "Never miss a critical event"}]},
        {"title": "System Features", "items": [{"title": "Cloud-native", "description": "Scales on demand"}, {"title": "API-first", "description": "Integrate in minutes"}]},
    ],
    "quote": [
        {"quote": "This tool transformed how we work. The results are incredible.", "author": "Sarah Chen", "role": "VP Engineering"},
        {"quote": "Best investment we made this year. ROI within 3 months.", "author": "Mike Johnson", "role": "CEO"},
    ],
    "comparison": [
        {"title": "Why Choose Us?", "left_label": "Traditional", "right_label": "AI-Powered", "left_items": ["Manual processing", "Hours of work", "Human error"], "right_items": ["Automated", "Instant results", "Error-free"]},
    ],
    "stat_row": [
        {"title": "Platform Metrics", "stats": [{"value": "10M+", "label": "Data Points"}, {"value": "99.9%", "label": "Uptime"}, {"value": "50ms", "label": "Response"}]},
    ],
    "timeline": [
        {"title": "Implementation Roadmap", "steps": [{"title": "Discovery", "description": "Analyze needs"}, {"title": "Setup", "description": "Configure platform"}, {"title": "Launch", "description": "Go live"}]},
    ],
    "callout": [
        {"title": "Pro Tip", "text": "Start with a pilot program to validate before full deployment.", "icon": "💡", "variant": "info"},
        {"title": "Warning", "text": "Ensure compliance before processing sensitive data.", "icon": "⚠️", "variant": "warning"},
    ],
    "split_features": [
        {"title": "Performance", "features": [{"icon": "⚡", "title": "Sub-100ms latency", "description": "Global edge servers"}, {"icon": "🔋", "title": "99.99% availability", "description": "Multi-region fallback"}]},
    ],
    "grid_cards": [
        {"title": "Platform Capabilities", "cards": [{"icon": "🤖", "title": "AI Models", "description": "Custom LLMs"}, {"icon": "📊", "title": "Analytics", "description": "Visual reports"}, {"icon": "🔒", "title": "Security", "description": "Role-based access"}]},
    ],
    "headline_subheadline": [
        {"headline": "The Next Chapter", "subheadline": "Announcing our $20M Series B funding round."},
        {"headline": "A New Way to Build", "subheadline": "Clean code interfaces for modern teams."},
    ],
    "definition": [
        {"term": "Responsive Web Design", "definition": "An approach to web design that makes web pages render well on a variety of devices and window or screen sizes.", "context": "Front-end Development"},
    ],
    "text_block": [
        {"title": "A Guide to Modularity", "body": "Modularity is the art of breaking a system into self-contained modules that can be modified independently."},
    ],
}

ARCHETYPES = [
    "educator", "thought_leader", "startup_pitch", "brand_storyteller",
    "data_analyst", "creator"
]

PLATFORMS = ["instagram_portrait", "instagram_square", "instagram_story", "tiktok_vertical", "linkedin_landscape"]
BRAND_COLORS = ["#6366F1", "#EF4444", "#10B981", "#F59E0B", "#8B5CF6", "#EC4899", "#06B6D4", "#84CC16"]
THEMES = ["editorial", "bold", "minimal", "dark", "vibrant", "natural"]

class RustMcpClient:
    def __init__(self, binary_path):
        self.process = subprocess.Popen(
            [binary_path, "mcp"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        self.msg_id = 1

    def call_tool(self, tool_name, arguments):
        payload = {
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": arguments
            },
            "id": self.msg_id
        }
        self.msg_id += 1
        self.process.stdin.write(json.dumps(payload) + "\n")
        self.process.stdin.flush()
        
        # Read response
        line = self.process.stdout.readline()
        response = json.loads(line)
        if "error" in response:
            raise Exception(response["error"])
        return response["result"]

    def close(self):
        self.process.terminate()

def run_test():
    binary_path = "./target/debug/slideforge-rust"
    if not os.path.exists(binary_path):
        print("Please build the project first with: cargo build")
        sys.exit(1)

    output_dir = "./test-drafts/full-scope-test-output-rust"
    os.makedirs(output_dir, exist_ok=True)

    print("Connecting to Rust MCP Server...")
    client = RustMcpClient(binary_path)

    print("Starting full-scope testing of Rust implementation...")
    all_slide_types = list(SLIDE_CONTENT.keys())

    # Generate 23 test configurations to match the scope of python test
    results = {"success": 0, "failed": 0, "errors": []}

    for idx in range(23):
        archetype = ARCHETYPES[idx % len(ARCHETYPES)]
        platform = PLATFORMS[idx % len(PLATFORMS)]
        color = BRAND_COLORS[idx % len(BRAND_COLORS)]
        theme = THEMES[idx % len(THEMES)]
        carousel_id = idx + 1

        # Select 4 random slide types
        slide_types = random.sample(all_slide_types, 4)

        print(f"[{carousel_id}/23] Testing {archetype} ({theme}) with slides: {', '.join(slide_types)}")

        try:
            # 1. Configure design
            client.call_tool("configure_design", {
                "primary_color": color,
                "visual_theme": theme,
                "archetype": archetype,
                "platform": platform,
                "brand_name": "NexusAI",
                "brand_handle": "@nexusai",
                "topic": f"Rust Scope Test: {carousel_id}",
                "url": "https://nexusai.io",
                "hashtags": ["#nexusai", f"#{archetype}", f"#{theme}"],
            })

            # 2. Generate slides
            slides = []
            for st in slide_types:
                content = random.choice(SLIDE_CONTENT[st])
                bg = random.choice(["light", "dark", "gradient"])
                params = {**content, "bg_style": bg}
                
                slide_res = client.call_tool("generate_slide", {
                    "slide_type": st,
                    "params": params
                })
                
                # Rust returns Value directly
                # Parse the "content" field of the JSON-RPC tool response which contains the tool output
                # rmcp wraps the returned Json value in an array of content blocks (typically a text block containing JSON)
                content_text = slide_res["content"][0]["text"]
                slide_data = json.loads(content_text)
                
                slides.append(slide_data)

            # 3. Render carousel
            html_path = os.path.join(output_dir, f"carousel_{carousel_id}_{archetype}.html")
            render_res = client.call_tool("render_carousel", {
                "slides": slides,
                "output_path": html_path
            })

            if os.path.exists(html_path):
                results["success"] += 1
            else:
                results["failed"] += 1
                results["errors"].append(f"Carousel {carousel_id} failed to write HTML file")

        except Exception as e:
            results["failed"] += 1
            results["errors"].append(f"Carousel {carousel_id} failed: {str(e)}")

    client.close()

    print("\n=== RUST FULL-SCOPE TEST RESULTS ===")
    print(f"Success: {results['success']}")
    print(f"Failed: {results['failed']}")
    if results["errors"]:
        print("Errors:")
        for err in results["errors"]:
            print(f"  - {err}")
    print(f"HTML files saved in: {output_dir}")

if __name__ == "__main__":
    run_test()
