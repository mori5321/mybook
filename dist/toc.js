// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="blog/en.html"><strong aria-hidden="true">1.</strong> Blog:en</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="blog/en/bit_operation_pattern_useful_in_algorithm_competition.html"><strong aria-hidden="true">1.1.</strong> 2024-02-25 :: Bit Operation Pattern Useful in Algorithm Competition</a></li><li class="chapter-item expanded "><a href="blog/en/cmake_with_neovim_lsp.html"><strong aria-hidden="true">1.2.</strong> 2024-02-24 :: Make Neovim LSP Work well for C++/OpenCV with CMake</a></li><li class="chapter-item expanded "><a href="blog/en/graceful_shutdown.html"><strong aria-hidden="true">1.3.</strong> 2024-02-12 :: Graceful Shutdown in your web application</a></li><li class="chapter-item expanded "><a href="blog/en/deploy_mdbook_on_cloudflare_page_and_github_action.html"><strong aria-hidden="true">1.4.</strong> 2024-01-03 :: Deploy mdbook on Cloudflare Page and Github Action</a></li><li class="chapter-item expanded "><a href="blog/en/learn_error_handling_from_golang.html"><strong aria-hidden="true">1.5.</strong> 2023-12-28 :: Learn Error Handling From Golang</a></li><li class="chapter-item expanded "><a href="blog/en/timezone.html"><strong aria-hidden="true">1.6.</strong> TBA:: Timezone Best Practice</a></li></ol></li><li class="chapter-item expanded "><a href="blog/ja.html"><strong aria-hidden="true">2.</strong> Blog:ja</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="blog/ja/writing_a_interpreter_in_go.html"><strong aria-hidden="true">2.1.</strong> 2024-04-32 :: Writing a interpreter in Go 総括</a></li><li class="chapter-item expanded "><a href="ror/concurrent_programming/04_typical_bug_and_issue_of_concurrent_programming/NOTE.html"><strong aria-hidden="true">2.2.</strong> ror</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
