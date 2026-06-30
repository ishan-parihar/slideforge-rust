#!/usr/bin/env python3
import os
import sys
import json
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
    # ── 13 new slide types ───────────────────────────────────────────────
    "metric_card": [
        {"value": "1.2M", "label": "Monthly Active Users", "trend": "+18%", "context": "Up from 1.0M last quarter"},
    ],
    "chart": [
        {"chart_type": "bar", "title": "Revenue Growth", "data": [{"label": "Q1", "value": 40}, {"label": "Q2", "value": 65}, {"label": "Q3", "value": 90}]},
    ],
    "scatter_plot": [
        {"title": "Performance vs Latency", "x_label": "Requests/s", "y_label": "Latency (ms)", "data": [{"x": 100, "y": 10}, {"x": 500, "y": 45}, {"x": 200, "y": 20}, {"x": 800, "y": 80}]},
    ],
    "gauge": [
        {"value": 73, "label": "CPU", "title": "System Load"},
    ],
    "radar_chart": [
        {"title": "Skill Assessment", "data": [{"label": "Speed", "value": 85}, {"label": "Accuracy", "value": 92}, {"label": "Scale", "value": 78}, {"label": "UX", "value": 88}]},
    ],
    "text_columns": [
        {"title": "Platform Overview", "columns": [{"heading": "Scalability", "body": "Automatically scales to handle millions of requests per second."}, {"heading": "Security", "body": "SOC 2 Type II certified with end-to-end encryption."}]},
    ],
    "section_divider": [
        {"kicker": "Chapter 02", "title": "From Insight to Execution", "subtitle": "A sharper operating model for the next growth cycle."},
    ],
    "problem_solution": [
        {"title": "The Execution Gap", "problem": "Teams collect insights but lose momentum before action.", "solution": "Convert every signal into a tracked next step.", "proof_points": [{"title": "Faster handoff", "description": "Decisions move directly into owners and timelines."}, {"title": "Clearer focus", "description": "Only the highest-leverage work enters the plan."}]},
    ],
    "myth_fact": [
        {"myth": "More dashboards automatically create better decisions.", "fact": "Better decisions come from fewer signals with clearer ownership.", "explanation": "The slide should help teams challenge a common assumption."},
    ],
    "checklist_action_plan": [
        {"title": "Launch Checklist", "items": [{"label": "Lock the customer promise"}, {"label": "Validate the funnel event map"}, {"label": "Assign owners to each risk"}, {"label": "Schedule the post-launch review"}]},
    ],
    "case_study_result": [
        {"client": "Northstar Labs", "challenge": "Launch cycles were slow and approval-heavy.", "solution": "A two-track workflow separated creative iteration from compliance review.", "results": [{"title": "42%", "description": "Faster campaign launch"}, {"title": "3.1x", "description": "More creative variants tested"}]},
    ],
    "pricing_plan": [
        {"title": "Choose a Plan", "plans": [{"name": "Starter", "price": "$49", "description": "For focused solo workflows."}, {"name": "Growth", "price": "$149", "description": "For teams scaling repeatable systems."}, {"name": "Scale", "price": "Custom", "description": "For enterprise governance and support."}]},
    ],
    "testimonial_avatar": [
        {"quote": "The team finally has one operating view for decisions and follow-through.", "author": "Maya Patel", "role": "COO, Northstar Labs"},
    ],
    "logo_cloud": [
        {"title": "Trusted by Modern Teams", "logos": ["Nexus", "Aster", "Northstar", "Helio", "Quanta", "SignalWorks"]},
    ],
    "faq": [
        {"title": "Common Questions", "questions": [{"question": "How fast can we launch?", "answer": "Most teams ship the first workflow in one week."}, {"question": "Can this support approvals?", "answer": "Yes, review steps can be mapped into the same flow."}]},
    ],
    "process_map": [
        {"title": "Operating Flow", "steps": [{"label": "Capture signal"}, {"label": "Prioritize opportunity"}, {"label": "Assign owner"}, {"label": "Review outcome"}]},
    ],
    "before_after_story": [
        {"title": "Before and After", "before": "Scattered updates across docs, chats, and dashboards.", "after": "One narrative that connects signal, decision, owner, and result.", "metric": "37% fewer status meetings after rollout."},
    ],
    "progress_rings": [
        {"title": "Project Completion", "description": "Overall progress across all workstreams", "items": [{"label": "Design", "value": 85, "color": "#6366F1"}, {"label": "Backend", "value": 60, "color": "#10B981"}, {"label": "Frontend", "value": 45, "color": "#F59E0B"}]},
    ],
    "comparison_bars": [
        {"title": "Our Platform vs Legacy", "description": "Direct performance comparison", "comparison": {"left": {"label": "Our Platform", "value": 95, "unit": "score"}, "right": {"label": "Legacy", "value": 62, "unit": "score"}}},
    ],
    "metric_grid": [
        {"title": "Key Metrics", "metrics": [{"value": "4.9", "label": "Rating", "trend": "+0.3"}, {"value": "99.9%", "label": "Uptime", "trend": "+0.1%"}, {"value": "120ms", "label": "Latency", "trend": "-15ms"}, {"value": "2.1M", "label": "Requests", "trend": "+42%"}]},
    ],
    "funnel_chart": [
        {"title": "Conversion Funnel", "steps": [{"label": "Visitors", "value": "100K"}, {"label": "Signups", "value": "25K"}, {"label": "Paying", "value": "8K"}]},
    ],
    "table": [
        {"title": "Quarterly Results", "headers": ["Quarter", "Revenue", "Growth"], "rows": [["Q1", "$4.2M", "+12%"], ["Q2", "$5.1M", "+21%"], ["Q3", "$6.8M", "+33%"]]},
    ],
    "metric_sparkline": [
        {"value": "892K", "label": "Monthly Visits", "trend": "+24%", "context": "Growing steadily since launch", "data": [20, 35, 45, 55, 62, 70, 85, 92]},
    ],
    "column_chart": [
        {"title": "Feature Adoption", "caption": "Percentage of users using each feature", "data": [{"label": "AI Assist", "value": 78}, {"label": "Reports", "value": 62}, {"label": "API", "value": 45}, {"label": "Mobile", "value": 90}]},
    ],
    "image_caption": [
        {"image_url": "https://images.unsplash.com/photo-1460925895917-afdab827c52f", "caption": "Workspace Setup", "description": "Ergonomic keyboard, mouse, and multi-monitor layout.", "layout": "image-top"}
    ],
    "image_headline": [
        {"image_url": "https://images.unsplash.com/photo-1460925895917-afdab827c52f", "headline": "Unlock Your Potential", "subheadline": "The journey starts here.", "overlay_position": "bottom"}
    ],
    "image_quote": [
        {"image_url": "https://images.unsplash.com/photo-1460925895917-afdab827c52f", "quote": "Simplicity is the ultimate sophistication.", "author": "Leonardo da Vinci", "role": "Polymath"}
    ],
    "image_callout": [
        {"image_url": "https://images.unsplash.com/photo-1460925895917-afdab827c52f", "callouts": [{"x": 30, "y": 40, "label": "Processor", "description": "Ultra-fast octa-core processor"}, {"x": 70, "y": 60, "label": "Battery", "description": "Long-lasting 5000mAh battery"}], "description": "Key hardware features highlights."}
    ],
    "image_stat": [
        {"image_url": "https://images.unsplash.com/photo-1460925895917-afdab827c52f", "stat_value": "99.9%", "stat_label": "Uptime Guaranteed", "description": "Enterprise SLA backing our cloud deployment.", "layout": "image-left"}
    ],
    "image_gallery": [
        {"images": [{"url": "https://images.unsplash.com/photo-1460925895917-afdab827c52f", "caption": "Design Phase"}, {"url": "https://images.unsplash.com/photo-1460925895917-afdab827c52f", "caption": "Development"}, {"url": "https://images.unsplash.com/photo-1460925895917-afdab827c52f", "caption": "Launch"}], "layout": "3-grid", "title": "Project Lifecycle", "section_caption": "End-to-end execution phases"}
    ],
    "image_collage": [
        {"images": [{"url": "https://images.unsplash.com/photo-1460925895917-afdab827c52f", "width": 120, "height": 100}, {"url": "https://images.unsplash.com/photo-1460925895917-afdab827c52f", "width": 110, "height": 130}, {"url": "https://images.unsplash.com/photo-1460925895917-afdab827c52f", "width": 130, "height": 90}], "style": "scattered", "title": "Creative Process", "section_caption": "Visual collage showcase"}
    ],
    "image_comparison": [
        {"before_image": "https://images.unsplash.com/photo-1460925895917-afdab827c52f", "after_image": "https://images.unsplash.com/photo-1460925895917-afdab827c52f", "before_label": "Before Optimization", "after_label": "After Optimization", "description": "Visual comparison of page layout rendering.", "divider_style": "arrow"}
    ],
}

ARCHETYPES = [
    "educator", "thought_leader", "startup_pitch", "brand_storyteller",
    "data_analyst", "creator"
]

PLATFORMS = ["instagram_portrait", "instagram_square", "instagram_story", "tiktok_vertical", "linkedin_landscape"]
BRAND_COLORS = ["#6366F1", "#EF4444", "#10B981", "#F59E0B", "#8B5CF6", "#EC4899", "#06B6D4", "#84CC16"]
THEMES = ["editorial", "bold", "minimal", "dark", "vibrant", "natural"]
BG_STYLES = ["light", "dark", "gradient"]
IMAGE_SLIDE_TYPES = [
    "image_caption", "image_headline", "image_quote", "image_callout",
    "image_stat", "image_gallery", "image_collage", "image_comparison",
]
IMAGE_FILTERS = ["none", "grayscale", "sepia", "duotone-warm", "duotone-cool", "high-contrast", "soft", "vintage"]
IMAGE_OVERLAYS = ["none", "gradient", "solid", "duotone", "vignette", "tint"]
IMAGE_FRAMES = ["sharp", "rounded", "squircle", "organic"]
IMAGE_POSITIONS = ["center", "top", "bottom", "left", "right", "full-bleed"]
TEST_IMAGES = [
    "https://images.unsplash.com/photo-1451187580459-43490279c0fa",
    "https://images.unsplash.com/photo-1518770660439-4636190af475",
    "https://images.unsplash.com/photo-1506748686214-e9df14d4d9d0",
]

IMAGE_VARIANT_OVERRIDES = {
    "image_caption": [
        {"layout": "image-top"},
        {"layout": "image-bottom"},
        {"layout": "image-left"},
        {"layout": "image-right"},
        {"layout": "image-overlay"},
    ],
    "image_headline": [
        {"overlay_position": "bottom"},
        {"overlay_position": "center"},
        {"overlay_position": "top"},
    ],
    "image_stat": [
        {"layout": "image-left"},
        {"layout": "image-right"},
        {"layout": "image-top"},
        {"layout": "image-bottom"},
    ],
    "image_gallery": [
        {"layout": "2-grid"},
        {"layout": "3-grid"},
        {"layout": "4-grid"},
        {"layout": "featured-1-2"},
        {"layout": "featured-2-1"},
    ],
    "image_collage": [
        {"style": "scattered"},
        {"style": "layered"},
        {"style": "geometric"},
        {"style": "editorial_stack"},
        {"style": "mosaic"},
        {"style": "filmstrip"},
    ],
    "image_comparison": [
        {"divider_style": "line"},
        {"divider_style": "arrow"},
    ],
}

class RustMcpClient:
    def __init__(self, binary_path):
        self.process = subprocess.Popen(
            [binary_path, "mcp"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        self.msg_id = 1
        self._handshake()

    def _handshake(self):
        init_payload = {
            "jsonrpc": "2.0",
            "id": self.msg_id,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-03-26",
                "capabilities": {},
                "clientInfo": {"name": "full-scope-test", "version": "0.1.0"},
            },
        }
        self.msg_id += 1
        self.process.stdin.write(json.dumps(init_payload) + "\n")
        self.process.stdin.flush()
        self.process.stdout.readline()

        notif = {"jsonrpc": "2.0", "method": "notifications/initialized"}
        self.process.stdin.write(json.dumps(notif) + "\n")
        self.process.stdin.flush()

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
    binary_path = "./target/release/slideforge-rust"
    if not os.path.exists(binary_path):
        print("Please build the project first with: cargo build")
        sys.exit(1)

    output_dir = "./test-drafts/full-scope-test-output-rust"
    os.makedirs(output_dir, exist_ok=True)
    for filename in os.listdir(output_dir):
        if filename.startswith("carousel_") and filename.endswith(".html"):
            os.remove(os.path.join(output_dir, filename))

    print("Connecting to Rust MCP Server...")
    client = RustMcpClient(binary_path)

    print("Starting full-scope testing of Rust implementation...")
    all_slide_types = list(SLIDE_CONTENT.keys())
    
    # Generate systematic test configurations to ensure full coverage:
    # - every slide type individually
    # - deterministic mixed carousels with no repeated slide type per carousel
    # - every theme's image treatment across all image slide types
    # - every explicit image filter override with frame/overlay/position coverage

    test_configs = []
    carousel_id = 1
    
    # Test each slide type individually with different themes
    for i, slide_type in enumerate(all_slide_types):
        theme = THEMES[i % len(THEMES)]
        archetype = ARCHETYPES[i % len(ARCHETYPES)]
        platform = PLATFORMS[i % len(PLATFORMS)]
        color = BRAND_COLORS[i % len(BRAND_COLORS)]
        
        test_configs.append({
            "id": carousel_id,
            "archetype": archetype,
            "platform": platform,
            "color": color,
            "theme": theme,
            "slide_types": [slide_type],
            "bg_style": BG_STYLES[i % len(BG_STYLES)],
            "global_params": {},
            "param_overrides": {},
            "description": f"Individual test for {slide_type}"
        })
        carousel_id += 1

    # Test deterministic mixed groups of 4 slide types (no repeats within carousel).
    mixed_groups = []
    for offset in range(0, len(all_slide_types), 4):
        group = all_slide_types[offset:offset + 4]
        if len(group) >= 3:
            mixed_groups.append(group)
    for offset in range(1, min(9, len(all_slide_types))):
        rotated = all_slide_types[offset:] + all_slide_types[:offset]
        mixed_groups.append(rotated[:4])

    for i, combo in enumerate(mixed_groups):
        theme = THEMES[i % len(THEMES)]
        archetype = ARCHETYPES[i % len(ARCHETYPES)]
        platform = PLATFORMS[i % len(PLATFORMS)]
        color = BRAND_COLORS[i % len(BRAND_COLORS)]
        
        test_configs.append({
            "id": carousel_id,
            "archetype": archetype,
            "platform": platform,
            "color": color,
            "theme": theme,
            "slide_types": list(combo),
            "bg_style": BG_STYLES[i % len(BG_STYLES)],
            "global_params": {},
            "param_overrides": {},
            "description": f"Combination test: {', '.join(combo)}"
        })
        carousel_id += 1

    # Test each theme's default image treatment on all image slide types.
    for i, theme in enumerate(THEMES):
        param_overrides = {}
        for j, slide_type in enumerate(IMAGE_SLIDE_TYPES):
            variants = IMAGE_VARIANT_OVERRIDES.get(slide_type, [{}])
            param_overrides[slide_type] = variants[(i + j) % len(variants)]
        test_configs.append({
            "id": carousel_id,
            "archetype": ARCHETYPES[i % len(ARCHETYPES)],
            "platform": PLATFORMS[i % len(PLATFORMS)],
            "color": BRAND_COLORS[i % len(BRAND_COLORS)],
            "theme": theme,
            "slide_types": IMAGE_SLIDE_TYPES,
            "bg_style": BG_STYLES[i % len(BG_STYLES)],
            "global_params": {},
            "param_overrides": param_overrides,
            "description": f"Theme image-treatment matrix: {theme}"
        })
        carousel_id += 1

    # Test background-image injection on non-image slides. These must stay full-bleed
    # and must never inherit rounded frame treatments from image themes.
    background_slide_types = ["hero", "feature", "quote", "grid_cards"]
    for i, theme in enumerate(THEMES):
        test_configs.append({
            "id": carousel_id,
            "archetype": ARCHETYPES[(i + 2) % len(ARCHETYPES)],
            "platform": PLATFORMS[(i + 1) % len(PLATFORMS)],
            "color": BRAND_COLORS[(i + 3) % len(BRAND_COLORS)],
            "theme": theme,
            "slide_types": background_slide_types,
            "bg_style": "dark" if theme in {"dark", "vibrant"} else "light",
            "global_params": {
                "background_image": TEST_IMAGES[i % len(TEST_IMAGES)],
                "image_opacity": 0.36,
            },
            "param_overrides": {},
            "description": f"Background image full-bleed matrix: {theme}"
        })
        carousel_id += 1

    # Test every explicit image filter override with high-quality frame/overlay ranges.
    for i, image_filter in enumerate(IMAGE_FILTERS):
        slide_types = [IMAGE_SLIDE_TYPES[(i + j) % len(IMAGE_SLIDE_TYPES)] for j in range(4)]
        theme = THEMES[i % len(THEMES)]
        archetype = ARCHETYPES[i % len(ARCHETYPES)]
        platform = PLATFORMS[i % len(PLATFORMS)]
        color = BRAND_COLORS[i % len(BRAND_COLORS)]

        test_configs.append({
            "id": carousel_id,
            "archetype": archetype,
            "platform": platform,
            "color": color,
            "theme": theme,
            "slide_types": slide_types,
            "bg_style": BG_STYLES[i % len(BG_STYLES)],
            "global_params": {
                "image_filter": image_filter,
                "image_overlay": IMAGE_OVERLAYS[i % len(IMAGE_OVERLAYS)],
                "image_frame": IMAGE_FRAMES[i % len(IMAGE_FRAMES)],
                "image_position": IMAGE_POSITIONS[i % len(IMAGE_POSITIONS)],
                "image_opacity": 0.32,
            },
            "param_overrides": {},
            "description": f"Explicit image filter test: {image_filter}"
        })
        carousel_id += 1
    
    # Run tests
    results = {"success": 0, "failed": 0, "errors": [], "slide_coverage": set()}
    
    configs_to_run = test_configs
    for config in configs_to_run:
        carousel_id = config["id"]
        archetype = config["archetype"]
        platform = config["platform"]
        color = config["color"]
        theme = config["theme"]
        slide_types = config["slide_types"]
        bg_style = config["bg_style"]
        
        print(f"[{carousel_id}/{len(configs_to_run)}] Testing {archetype} ({theme}) with slides: {', '.join(slide_types)}")
        
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
            
            # 2. Generate slides (no repeats within carousel)
            slides = []
            for st in slide_types:
                # Use first content item for this slide type
                content = SLIDE_CONTENT[st][0]
                params = {
                    **content,
                    **config.get("global_params", {}),
                    **config.get("param_overrides", {}).get(st, {}),
                    "bg_style": bg_style,
                }
                
                slide_res = client.call_tool("generate_slide", {
                    "slide_type": st,
                    "params": params
                })
                
                # Rust returns Value directly
                content_text = slide_res["content"][0]["text"]
                slide_data = json.loads(content_text)
                
                slides.append(slide_data)
                results["slide_coverage"].add(st)
            
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
    print(f"Slide type coverage: {len(results['slide_coverage'])}/{len(all_slide_types)}")
    print(f"Covered slide types: {', '.join(sorted(results['slide_coverage']))}")
    
    # Check for missing slide types
    missing_types = set(all_slide_types) - results["slide_coverage"]
    if missing_types:
        print(f"Missing slide types: {', '.join(sorted(missing_types))}")
    
    if results["errors"]:
        print("Errors:")
        for err in results["errors"]:
            print(f"  - {err}")
    print(f"HTML files saved in: {output_dir}")

if __name__ == "__main__":
    run_test()
