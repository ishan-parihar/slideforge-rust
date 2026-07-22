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
    "metric_grid": [
        {"title": "Key Metrics", "items": [{"label": "Monthly Active Users", "value": "1.2M", "trend": "+18%"}]},
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
        {"client": "Northstar Labs", "challenge": "Launch cycles were slow and approval-heavy.", "solution": "A two-track workflow separated creative iteration from compliance review.", "results": [{"icon": "↗", "title": "42%", "description": "Faster campaign launch"}, {"icon": "✦", "title": "3.1x", "description": "More creative variants tested"}]},
    ],
    "pricing_plan": [
        {"title": "Choose a Plan", "plans": [{"icon": "S", "name": "Starter", "price": "$49", "description": "For focused solo workflows."}, {"icon": "G", "name": "Growth", "price": "$149", "description": "For teams scaling repeatable systems."}, {"icon": "∞", "name": "Scale", "price": "Custom", "description": "For enterprise governance and support."}]},
    ],
    "testimonial_avatar": [
        {"quote": "The team finally has one operating view for decisions and follow-through.", "author": "Maya Patel", "role": "COO, Northstar Labs"},
    ],
    "logo_cloud": [
        {"title": "Trusted by Modern Teams", "logos": [{"icon": "N", "name": "Nexus"}, {"icon": "A", "name": "Aster"}, {"icon": "✦", "name": "Northstar"}, {"icon": "H", "name": "Helio"}, {"icon": "Q", "name": "Quanta"}, {"icon": "S", "name": "SignalWorks"}]},
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
    "qr_destination": [
        {
            "destination_url": "https://nexusai.io/blog/agentic-slide-workflows",
            "heading": "Read the full workflow",
            "caption": "A practical guide to turning carousel attention into owned traffic.",
            "cta_text": "Scan to read",
            "short_url": "nexusai.io/guide",
            "incentive_text": "Includes templates and examples.",
            "variant": "full-conversion",
        }
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
ASPECT_RATIOS = ["4:5", "9:16", "3:4", "1:1"]
PLATFORM_RATIO_OVERRIDES = {
    "instagram_portrait": ["4:5", "3:4", "1:1"],
    "instagram_square": ["1:1", "4:5"],
    "instagram_story": ["9:16", "3:4"],
    "tiktok_vertical": ["9:16"],
    "linkedin_landscape": ["4:5", "1:1"],
}
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
    
    # Generate one carousel per slide type. Each carousel contains exactly one
    # slide so manual review can focus on the full design surface of that type.
    # Theme, archetype, platform, image treatment, and background-image settings
    # are spread across slide types instead of repeating slide combinations.

    test_configs = []
    carousel_id = 1

    for i, slide_type in enumerate(all_slide_types):
        theme = THEMES[i % len(THEMES)]
        archetype = ARCHETYPES[i % len(ARCHETYPES)]
        platform = PLATFORMS[i % len(PLATFORMS)]
        allowed = PLATFORM_RATIO_OVERRIDES.get(platform, ["4:5"])
        aspect_ratio = allowed[i % len(allowed)]
        color = BRAND_COLORS[i % len(BRAND_COLORS)]
        overrides = {}
        if slide_type in IMAGE_VARIANT_OVERRIDES:
            variants = IMAGE_VARIANT_OVERRIDES[slide_type]
            overrides.update(variants[i % len(variants)])
            overrides.update({
                "image_filter": IMAGE_FILTERS[i % len(IMAGE_FILTERS)],
                "image_overlay": IMAGE_OVERLAYS[i % len(IMAGE_OVERLAYS)],
                "image_frame": IMAGE_FRAMES[i % len(IMAGE_FRAMES)],
                "image_position": IMAGE_POSITIONS[i % len(IMAGE_POSITIONS)],
            })

        global_params = {}
        if slide_type not in IMAGE_SLIDE_TYPES and i % 7 == 0:
            global_params = {
                "background_image": TEST_IMAGES[i % len(TEST_IMAGES)],
                "image_opacity": 0.36,
            }

        test_configs.append({
            "id": carousel_id,
            "archetype": archetype,
            "platform": platform,
            "aspect_ratio": aspect_ratio,
            "color": color,
            "theme": theme,
            "slide_types": [slide_type],
            "bg_style": BG_STYLES[i % len(BG_STYLES)],
            "global_params": global_params,
            "param_overrides": {slide_type: overrides},
            "description": f"Single-slide test for {slide_type}"
        })
        carousel_id += 1
    
    # Run tests
    results = {
        "success": 0,
        "failed": 0,
        "errors": [],
        "slide_coverage": set(),
        "ratio_coverage": set()
    }
    
    configs_to_run = test_configs
    for config in configs_to_run:
        carousel_id = config["id"]
        archetype = config["archetype"]
        platform = config["platform"]
        aspect_ratio = config["aspect_ratio"]
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
                "aspect_ratio": aspect_ratio,
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
                results["ratio_coverage"].add(aspect_ratio)
            
            # 3. Render carousel
            html_path = os.path.join(output_dir, f"carousel_{carousel_id}_{archetype}.html")
            render_res = client.call_tool("render_carousel", {
                "slides": slides,
                "output_path": html_path,
                "platform": platform,
                "aspect_ratio": aspect_ratio,
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
    print(f"Ratio coverage: {len(results['ratio_coverage'])}/{len(ASPECT_RATIOS)}")
    print(f"Covered aspect ratios: {', '.join(sorted(results['ratio_coverage']))}")
    
    # Check for missing slide types
    missing_types = set(all_slide_types) - results["slide_coverage"]
    if missing_types:
        print(f"Missing slide types: {', '.join(sorted(missing_types))}")
    
    # Check for missing aspect ratios
    missing_ratios = set(ASPECT_RATIOS) - results["ratio_coverage"]
    if missing_ratios:
        results["failed"] += 1
        results["errors"].append(f"Missing aspect ratios: {', '.join(sorted(missing_ratios))}")
        print(f"Missing aspect ratios: {', '.join(sorted(missing_ratios))}")
    
    if results["errors"]:
        print("Errors:")
        for err in results["errors"]:
            print(f"  - {err}")
    print(f"HTML files saved in: {output_dir}")
    
    # Exit with failure if any failed
    if results["failed"] > 0:
        sys.exit(1)

if __name__ == "__main__":
    run_test()
