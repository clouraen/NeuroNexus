// CSS theme for cyberpunk neon aesthetic
pub const CSS: &str = r#"
@import url('https://fonts.googleapis.com/css2?family=Orbitron:wght@400;700;900&family=Roboto:wght@400;600&display=swap');

:root {
    /* Background Colors */
    --bg-black: #000000;
    --bg-dark: #0a0a0a;
    
    /* Neon Colors - Primary Palette */
    --neon-cyan: #00ffff;
    --neon-cyan-bright: #0ff0ff;
    --neon-cyan-light: #00d9ff;
    --neon-purple: #9d4edd;
    --neon-purple-bright: #c77dff;
    --neon-purple-light: #e0aaff;
    --neon-orange: #ff6b35;
    --neon-pink: #ff10f0;
    --hot-pink: #ff10f0;
    
    /* Supporting Colors */
    --muted-purple: rgba(157, 78, 221, 0.5);
    --gold: #ffd700;
    --gold-light: #ffed4e;
    --white: #ffffff;
    --light-gray: #cccccc;
    
    /* Typography */
    --font-heading: 'Orbitron', 'Courier New', monospace;
    --font-body: 'Roboto', 'Arial', sans-serif;
    --font-mono: 'Courier New', monospace;
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    background: var(--bg-black);
    color: var(--neon-purple-light);
    font-family: var(--font-body);
    overflow-x: hidden;
    line-height: 1.5;
}

.app-container {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    background: linear-gradient(135deg, var(--bg-black) 0%, var(--bg-dark) 100%);
}

/* Status Bar - Header Section */
.status-bar {
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    padding: 1rem 2rem;
    height: 4rem;
    background: rgba(10, 10, 10, 0.95);
    backdrop-filter: blur(10px);
    border-bottom: 2px solid var(--neon-purple);
    box-shadow: 0 2px 15px rgba(157, 78, 221, 0.3);
    position: relative;
    z-index: 100;
}

.status-bar-left {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    justify-self: start;
}

.status-logo {
    display: flex;
    align-items: center;
    gap: 0.75rem;
}

.logo-icon {
    font-size: 1.5rem;
    color: var(--neon-cyan);
    text-shadow: 0 0 10px var(--neon-cyan);
    animation: glow-pulse 2s ease-in-out infinite;
}

@keyframes glow-pulse {
    0%, 100% {
        text-shadow: 0 0 10px var(--neon-cyan);
    }
    50% {
        text-shadow: 0 0 20px var(--neon-cyan), 0 0 30px var(--neon-cyan);
    }
}

.logo-text {
    font-size: 1.8rem;
    font-weight: bold;
    color: var(--neon-cyan);
    text-shadow: 0 0 10px var(--neon-cyan), 0 0 20px var(--neon-cyan);
    letter-spacing: 0.15em;
    font-family: var(--font-heading);
}

.brain-icon {
    font-size: 1.5rem;
    color: var(--neon-cyan);
    text-shadow: 0 0 15px var(--neon-cyan);
    animation: brain-pulse 2s ease-in-out infinite;
    margin-right: 0.5rem;
}

@keyframes brain-pulse {
    0%, 100% {
        text-shadow: 0 0 10px var(--neon-cyan);
        transform: scale(1.0);
    }
    50% {
        text-shadow: 0 0 20px var(--neon-cyan), 0 0 30px var(--neon-cyan);
        transform: scale(1.05);
    }
}

.status-bar-center {
    display: flex;
    justify-content: center;
    align-items: center;
}

.status-time {
    font-size: 1.2rem;
    font-weight: 600;
    color: var(--neon-purple-light);
    text-shadow: 0 0 8px var(--neon-purple);
    font-family: var(--font-mono);
    letter-spacing: 0.1em;
    cursor: pointer;
    user-select: none;
    transition: all 0.3s ease;
}

.status-time:hover {
    color: var(--neon-purple-bright);
    text-shadow: 0 0 12px var(--neon-purple-bright);
}

.status-time.active {
    animation: timer-pulse 3s ease-in-out infinite;
}

@keyframes timer-pulse {
    0%, 100% {
        text-shadow: 0 0 8px var(--neon-purple);
    }
    50% {
        text-shadow: 0 0 15px var(--neon-purple), 0 0 25px var(--neon-purple);
    }
}

.status-bar-right {
    display: flex;
    align-items: center;
    justify-self: end;
}

.status-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
}

.info-item {
    color: var(--white);
    font-size: 0.9rem;
    font-family: var(--font-body);
}

.online-status {
    color: var(--hot-pink);
    text-shadow: 0 0 10px var(--hot-pink);
    animation: blink 2s ease-in-out infinite;
    font-weight: 600;
}

@keyframes blink {
    0%, 100% {
        opacity: 1;
    }
    50% {
        opacity: 0.6;
    }
}

.info-separator {
    color: var(--neon-purple);
    opacity: 0.5;
}

.version-text {
    color: var(--white);
    font-size: 0.9rem;
}

/* Page Container */
.page-container {
    flex: 1;
    padding: 2rem;
    padding-bottom: 6rem;
    overflow-y: auto;
}

.page-title {
    font-size: 2.5rem;
    text-transform: uppercase;
    color: var(--neon-purple);
    text-shadow: 0 0 10px var(--neon-purple), 0 0 20px var(--neon-purple);
    margin-bottom: 2rem;
    letter-spacing: 0.2em;
    font-family: var(--font-heading);
    font-weight: 700;
}

/* Section Titles */
.section-title {
    font-size: 1.5rem;
    text-transform: uppercase;
    color: var(--neon-purple);
    text-shadow: 0 0 10px var(--neon-purple);
    margin: 2rem 0 1rem 0;
    letter-spacing: 0.1em;
    font-family: var(--font-heading);
    font-weight: 700;
}

/* Panel Titles */
.panel-title {
    font-size: 1.8rem;
    text-transform: uppercase;
    color: var(--neon-purple);
    text-shadow: 0 0 10px var(--neon-purple);
    margin-bottom: 1rem;
    letter-spacing: 0.1em;
    font-family: var(--font-heading);
    font-weight: 700;
}

/* Neon Button */
.neon-button {
    background: transparent;
    border: 2px solid var(--neon-purple);
    border-radius: 16px;
    color: var(--neon-purple);
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    font-family: inherit;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    cursor: pointer;
    transition: all 0.3s ease;
    text-shadow: 0 0 10px var(--neon-purple);
    box-shadow: 0 0 10px rgba(157, 78, 221, 0.3), inset 0 0 10px rgba(157, 78, 221, 0.1);
}

.neon-button:hover {
    border-color: var(--neon-purple-bright);
    color: var(--neon-purple-bright);
    box-shadow: 0 0 20px var(--neon-purple-bright), 0 0 30px var(--neon-purple-bright), inset 0 0 20px rgba(199, 125, 255, 0.2);
    transform: translateY(-2px);
}

.neon-button-primary {
    border-color: var(--neon-purple);
    color: var(--neon-purple);
}

.neon-button-secondary {
    border-color: var(--neon-cyan);
    color: var(--neon-cyan);
    text-shadow: 0 0 10px var(--neon-cyan);
    box-shadow: 0 0 10px rgba(0, 255, 255, 0.3), inset 0 0 10px rgba(0, 255, 255, 0.1);
}

.neon-button-secondary:hover {
    border-color: var(--neon-cyan-bright);
    color: var(--neon-cyan-bright);
    box-shadow: 0 0 20px var(--neon-cyan-bright), 0 0 30px var(--neon-cyan-bright);
}

.neon-button-danger {
    border-color: var(--neon-orange);
    color: var(--neon-orange);
    text-shadow: 0 0 10px var(--neon-orange);
    box-shadow: 0 0 10px rgba(255, 107, 53, 0.3), inset 0 0 10px rgba(255, 107, 53, 0.1);
}

.neon-button-danger:hover {
    border-color: var(--neon-orange);
    color: var(--neon-orange);
    box-shadow: 0 0 25px var(--neon-orange), 0 0 35px var(--neon-orange);
    transform: translateY(-2px);
}

/* Neon Input */
.neon-input {
    background: rgba(10, 10, 10, 0.8);
    border: 2px solid var(--neon-purple);
    border-radius: 14px;
    color: var(--neon-purple-light);
    padding: 0.75rem 1rem;
    font-size: 1rem;
    font-family: inherit;
    width: 100%;
    transition: all 0.3s ease;
    box-shadow: 0 0 10px rgba(157, 78, 221, 0.2);
}

.neon-input:focus {
    outline: none;
    border-color: var(--neon-purple-bright);
    box-shadow: 0 0 20px var(--neon-purple-bright), 0 0 30px var(--neon-purple-bright);
}

.neon-input::placeholder {
    color: rgba(157, 78, 221, 0.5);
}

/* Cyber Card */
.cyber-card {
    background: linear-gradient(135deg, rgba(10, 10, 10, 0.9) 0%, rgba(0, 0, 0, 0.9) 100%);
    border: 1px solid var(--neon-purple);
    border-radius: 20px;
    padding: 1.5rem;
    transition: all 0.3s ease;
    box-shadow: 0 0 15px rgba(157, 78, 221, 0.2);
}

.cyber-card:hover {
    border-color: var(--neon-purple-bright);
    box-shadow: 0 0 25px rgba(199, 125, 255, 0.4);
    transform: translateY(-2px);
}

/* Tab Bar / Dock Bar */
.tab-bar {
    display: flex;
    justify-content: space-around;
    align-items: center;
    padding: 0.75rem 0;
    height: 5rem;
    background: rgba(10, 10, 10, 0.95);
    backdrop-filter: blur(10px);
    border-top: 2px solid var(--neon-purple);
    box-shadow: 0 -5px 20px rgba(157, 78, 221, 0.3);
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    width: 100%;
    z-index: 1000;
}

.tab-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.4rem;
    padding: 0.75rem 1.25rem;
    color: var(--neon-purple);
    text-decoration: none;
    transition: all 0.3s ease;
    border-radius: 12px;
    position: relative;
    min-width: 80px;
}

.tab-item:hover {
    color: var(--neon-purple-bright);
    text-shadow: 0 0 10px var(--neon-purple-bright);
    background: rgba(157, 78, 221, 0.1);
}

.tab-item.active {
    color: var(--neon-purple-bright);
    text-shadow: 0 0 15px var(--neon-purple-bright);
    background: rgba(199, 125, 255, 0.15);
}

.tab-item-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.4rem;
}

.tab-icon {
    font-size: 1.6rem;
    filter: drop-shadow(0 0 5px currentColor);
    transition: transform 0.3s ease;
}

.tab-item:hover .tab-icon {
    transform: scale(1.1);
}

.tab-label {
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.15em;
    font-weight: 600;
}

.tab-indicator {
    position: absolute;
    bottom: -2px;
    left: 50%;
    transform: translateX(-50%);
    width: 60%;
    height: 3px;
    background: linear-gradient(90deg, transparent, var(--neon-purple-bright), transparent);
    box-shadow: 0 0 10px var(--neon-purple-bright), 0 0 20px var(--neon-purple-bright);
    border-radius: 6px;
    animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
    0%, 100% {
        opacity: 1;
        box-shadow: 0 0 10px var(--neon-purple-bright), 0 0 20px var(--neon-purple-bright);
    }
    50% {
        opacity: 0.7;
        box-shadow: 0 0 15px var(--neon-purple-bright), 0 0 30px var(--neon-purple-bright);
    }
}

/* Dashboard Grid */
.dashboard-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
    margin-bottom: 3rem;
}

.stat-card {
    text-align: center;
    border-radius: 16px;
}

.stat-value {
    font-size: 3rem;
    color: var(--neon-cyan);
    text-shadow: 0 0 20px var(--neon-cyan);
    margin-bottom: 0.5rem;
}

.stat-label {
    font-size: 1rem;
    color: var(--neon-purple-light);
    text-transform: uppercase;
    letter-spacing: 0.1em;
}

/* Category Grid */
.category-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: 1rem;
}

.category-card {
    text-align: center;
    cursor: pointer;
    border-radius: 16px;
}

.category-icon {
    font-size: 3rem;
    margin-bottom: 0.5rem;
}

.category-name {
    font-size: 1rem;
    color: var(--neon-purple-light);
    text-transform: uppercase;
    letter-spacing: 0.1em;
}

/* Question Card */
.question-card {
    margin-bottom: 1rem;
    border-radius: 16px;
}

.question-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 1rem;
}

.question-subject {
    color: var(--neon-cyan);
    font-weight: bold;
    text-transform: uppercase;
}

.question-difficulty {
    color: var(--neon-pink);
    font-size: 0.9rem;
}

.question-preview {
    color: var(--neon-purple-light);
    line-height: 1.6;
}

/* Essay Card */
.essay-card {
    margin-bottom: 1rem;
    border-radius: 16px;
}

.essay-header {
    display: flex;
    justify-content: space-between;
    align-items: start;
    margin-bottom: 1rem;
}

.essay-title {
    color: var(--neon-purple-light);
    font-size: 1.2rem;
    margin-bottom: 0.5rem;
}

.essay-exam-type {
    color: var(--neon-cyan);
    font-size: 0.9rem;
    text-transform: uppercase;
}

.essay-info {
    display: flex;
    justify-content: space-between;
}

.essay-status {
    color: var(--neon-pink);
}

.essay-score {
    color: var(--gold);
    font-weight: bold;
    text-shadow: 0 0 10px var(--gold);
}

/* Profile Card */
.profile-card {
    margin-bottom: 2rem;
    border-radius: 18px;
}

.profile-header {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    margin-bottom: 2rem;
}

.profile-avatar {
    font-size: 3rem;
    width: 80px;
    height: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 2px solid var(--neon-purple);
    border-radius: 50%;
    box-shadow: 0 0 20px rgba(157, 78, 221, 0.5);
}

.profile-info h2 {
    color: var(--white);
    font-size: 1.2rem;
    margin-bottom: 0.5rem;
    font-family: var(--font-body);
}

.profile-info p {
    color: var(--light-gray);
    font-size: 0.9rem;
}

.profile-stats {
    display: flex;
    gap: 2rem;
    margin-top: 2rem;
}

.stat-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
}

.stat-number {
    font-size: 2rem;
    color: var(--neon-cyan);
    text-shadow: 0 0 10px var(--neon-cyan);
    font-weight: 700;
}

.stat-desc {
    font-size: 0.9rem;
    color: var(--neon-purple);
    margin-top: 0.5rem;
}

/* Progress Bar */
.neon-progress-container {
    margin: 1rem 0;
}

.progress-label {
    color: var(--neon-purple-light);
    margin-bottom: 0.5rem;
    font-size: 0.9rem;
}

.neon-progress-bar {
    width: 100%;
    height: 20px;
    background: rgba(10, 10, 10, 0.8);
    border: 1px solid var(--neon-purple);
    border-radius: 12px;
    overflow: hidden;
    position: relative;
}

.neon-progress-fill {
    height: 100%;
    background: linear-gradient(90deg, var(--neon-purple) 0%, var(--neon-purple-bright) 100%);
    border-radius: 12px;
    box-shadow: 0 0 10px var(--neon-purple), 0 0 20px var(--neon-purple);
    transition: width 0.3s ease;
}

.progress-text {
    text-align: right;
    margin-top: 0.25rem;
    color: var(--neon-purple-light);
    font-size: 0.9rem;
}

/* Section Titles */
.section-title {
    font-size: 1.5rem;
    text-transform: uppercase;
    color: var(--neon-purple);
    text-shadow: 0 0 10px var(--neon-purple);
    margin: 2rem 0 1rem 0;
    letter-spacing: 0.1em;
}

/* Search Section */
.search-section {
    margin-bottom: 2rem;
}

/* Page Header */
.page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
}

/* Settings */
.settings-card {
    margin-top: 1rem;
    border-radius: 18px;
}

.setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 0;
    border-bottom: 1px solid rgba(157, 78, 221, 0.2);
}

.setting-item:last-child {
    border-bottom: none;
}

.setting-label {
    color: var(--white);
    font-size: 1rem;
    font-family: var(--font-body);
}

.setting-toggle {
    color: var(--neon-purple);
    font-size: 1.2rem;
    text-shadow: 0 0 8px var(--neon-purple);
    cursor: pointer;
    transition: all 0.3s ease;
    border-radius: 10px;
}

.setting-toggle:hover {
    transform: scale(1.1);
    text-shadow: 0 0 12px var(--neon-purple-bright);
}

.setting-toggle.disabled {
    color: var(--muted-purple);
    text-shadow: none;
}

/* Two-Panel Layout */
.two-panel-layout {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
    margin-bottom: 2rem;
}

.panel-container {
    display: flex;
    flex-direction: column;
}

.panel-card {
    background: linear-gradient(135deg, rgba(10, 10, 10, 0.9) 0%, rgba(0, 0, 0, 0.9) 100%);
    border: 1px solid var(--neon-purple);
    border-radius: 18px;
    padding: 2rem;
    box-shadow: 0 0 15px rgba(157, 78, 221, 0.2);
    transition: all 0.3s ease;
}

/* Brain Motif */
.brain-watermark {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 40vw;
    height: 40vw;
    max-width: 500px;
    max-height: 500px;
    opacity: 0.04;
    pointer-events: none;
    z-index: 0;
    color: var(--neon-purple);
    font-size: 20rem;
}

.brain-watermark svg {
    width: 100%;
    height: 100%;
    filter: drop-shadow(0 0 20px var(--neon-purple));
}

/* Scrollbar Styling */
::-webkit-scrollbar {
    width: 10px;
}

::-webkit-scrollbar-track {
    background: var(--bg-dark);
}

::-webkit-scrollbar-thumb {
    background: var(--neon-purple);
    border-radius: 5px;
}

::-webkit-scrollbar-thumb:hover {
    background: var(--neon-purple-bright);
    box-shadow: 0 0 10px var(--neon-purple-bright);
}

/* Sidebar Navigation */
.sidebar {
    width: 280px;
    background: rgba(10, 10, 10, 0.95);
    backdrop-filter: blur(10px);
    border-right: 2px solid var(--neon-purple);
    box-shadow: 5px 0 20px rgba(157, 78, 221, 0.2);
    display: flex;
    flex-direction: column;
    height: 100vh;
    position: fixed;
    left: 0;
    top: 0;
    z-index: 100;
}

.sidebar-header {
    padding: 2rem 1.5rem;
    border-bottom: 2px solid var(--neon-purple);
    box-shadow: 0 2px 10px rgba(157, 78, 221, 0.2);
}

.sidebar-logo {
    font-size: 1.8rem;
    font-weight: bold;
    color: var(--neon-cyan);
    text-shadow: 0 0 15px var(--neon-cyan), 0 0 30px var(--neon-cyan);
    letter-spacing: 0.2em;
    margin-bottom: 0.5rem;
}

.sidebar-subtitle {
    font-size: 0.75rem;
    color: var(--neon-purple-light);
    text-transform: uppercase;
    letter-spacing: 0.2em;
    opacity: 0.7;
}

.sidebar-nav {
    flex: 1;
    padding: 1rem 0;
    overflow-y: auto;
}

.nav-item {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem 1.5rem;
    color: var(--neon-purple-light);
    text-decoration: none;
    transition: all 0.3s ease;
    border-left: 3px solid transparent;
    border-radius: 12px;
    position: relative;
}

.nav-item:hover {
    background: rgba(157, 78, 221, 0.1);
    border-left-color: var(--neon-purple);
    color: var(--neon-purple-bright);
}

.nav-item.active {
    background: rgba(199, 125, 255, 0.15);
    border-left-color: var(--neon-purple-bright);
    color: var(--neon-purple-bright);
}

.nav-item.active::before {
    content: '';
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 3px;
    background: var(--neon-purple-bright);
    box-shadow: 0 0 10px var(--neon-purple-bright);
}

.nav-item-icon {
    font-size: 1.5rem;
    filter: drop-shadow(0 0 5px currentColor);
    min-width: 24px;
    text-align: center;
}

.nav-item-content {
    flex: 1;
}

.nav-item-label {
    font-size: 1rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 0.25rem;
}

.nav-item-description {
    font-size: 0.75rem;
    opacity: 0.6;
    color: var(--neon-purple-light);
}

.sidebar-footer {
    padding: 1.5rem;
    border-top: 2px solid var(--neon-purple);
    box-shadow: 0 -2px 10px rgba(157, 78, 221, 0.2);
}

.sidebar-stats {
    display: flex;
    justify-content: space-around;
    gap: 1rem;
}

.stat-mini {
    display: flex;
    flex-direction: column;
    align-items: center;
    flex: 1;
}

.stat-value-mini {
    font-size: 1.5rem;
    color: var(--neon-cyan);
    text-shadow: 0 0 10px var(--neon-cyan);
    font-weight: bold;
}

.stat-label-mini {
    font-size: 0.7rem;
    color: var(--neon-purple-light);
    opacity: 0.8;
    text-transform: uppercase;
    letter-spacing: 0.1em;
}

/* Navigation Menu (Mobile/Overlay) */
.nav-menu-container {
    position: relative;
}

.nav-menu-toggle {
    background: rgba(10, 10, 10, 0.9);
    border: 2px solid var(--neon-purple);
    border-radius: 12px;
    color: var(--neon-purple);
    padding: 0.75rem 1rem;
    font-size: 1.2rem;
    cursor: pointer;
    transition: all 0.3s ease;
    text-shadow: 0 0 10px var(--neon-purple);
    box-shadow: 0 0 10px rgba(157, 78, 221, 0.3);
}

.nav-menu-toggle:hover {
    border-color: var(--neon-purple-bright);
    color: var(--neon-purple-bright);
    box-shadow: 0 0 20px rgba(199, 125, 255, 0.5);
}

.nav-menu-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.8);
    backdrop-filter: blur(5px);
    z-index: 998;
    animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
    from {
        opacity: 0;
    }
    to {
        opacity: 1;
    }
}

.nav-menu {
    position: fixed;
    top: 0;
    right: 0;
    width: 320px;
    height: 100vh;
    background: rgba(10, 10, 10, 0.98);
    backdrop-filter: blur(15px);
    border-left: 2px solid var(--neon-purple);
    box-shadow: -5px 0 30px rgba(157, 78, 221, 0.4);
    z-index: 999;
    display: flex;
    flex-direction: column;
    animation: slideIn 0.3s ease;
    overflow-y: auto;
}

@keyframes slideIn {
    from {
        transform: translateX(100%);
    }
    to {
        transform: translateX(0);
    }
}

.nav-menu-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 2px solid var(--neon-purple);
}

.nav-menu-header h3 {
    font-size: 1.5rem;
    color: var(--neon-cyan);
    text-shadow: 0 0 10px var(--neon-cyan);
    text-transform: uppercase;
    letter-spacing: 0.15em;
}

.nav-menu-close {
    background: transparent;
    border: 2px solid var(--neon-purple);
    border-radius: 12px;
    color: var(--neon-purple);
    padding: 0.5rem 0.75rem;
    font-size: 1.2rem;
    cursor: pointer;
    transition: all 0.3s ease;
}

.nav-menu-close:hover {
    border-color: var(--neon-purple-bright);
    color: var(--neon-purple-bright);
    box-shadow: 0 0 15px rgba(199, 125, 255, 0.5);
}

.menu-link {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1.25rem 1.5rem;
    color: var(--neon-purple-light);
    text-decoration: none;
    transition: all 0.3s ease;
    border-bottom: 1px solid rgba(157, 78, 221, 0.1);
}

.menu-link:hover {
    background: rgba(157, 78, 221, 0.1);
    color: var(--neon-purple-bright);
    padding-left: 2rem;
}

.menu-link-icon {
    font-size: 1.5rem;
    filter: drop-shadow(0 0 5px currentColor);
    min-width: 28px;
}

.menu-link-label {
    flex: 1;
    font-size: 1rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.menu-link-arrow {
    color: var(--neon-purple);
    opacity: 0.5;
    transition: all 0.3s ease;
}

.menu-link:hover .menu-link-arrow {
    opacity: 1;
    transform: translateX(5px);
    color: var(--neon-purple-bright);
}

.nav-menu-divider {
    height: 2px;
    background: linear-gradient(90deg, transparent, var(--neon-purple), transparent);
    margin: 1rem 0;
    box-shadow: 0 0 5px var(--neon-purple);
}

.nav-menu-section {
    padding: 1rem 1.5rem;
}

.nav-menu-section h4 {
    font-size: 0.9rem;
    color: var(--neon-cyan);
    text-transform: uppercase;
    letter-spacing: 0.15em;
    margin-bottom: 0.75rem;
    text-shadow: 0 0 8px var(--neon-cyan);
}

.nav-menu-placeholder {
    color: var(--neon-purple-light);
    opacity: 0.5;
    font-size: 0.85rem;
    font-style: italic;
    padding: 0.5rem 0;
}

/* Layout with Sidebar */
.app-with-sidebar {
    display: flex;
    min-height: 100vh;
}

.main-content-with-sidebar {
    margin-left: 280px;
    flex: 1;
    display: flex;
    flex-direction: column;
}

/* Responsive adjustments */
@media (max-width: 768px) {
    .sidebar {
        display: none;
    }
    
    .main-content-with-sidebar {
        margin-left: 0;
    }
}

/* Question Detail */
.question-detail {
    max-width: 800px;
    margin: 0 auto;
}

.question-statement {
    background: rgba(10, 10, 10, 0.8);
    border: 1px solid var(--neon-purple);
    border-radius: 16px;
    padding: 1.5rem;
    margin: 1.5rem 0;
    line-height: 1.8;
    font-size: 1.1rem;
}

.alternatives-section {
    margin: 2rem 0;
}

.alternative-item {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    margin: 0.5rem 0;
    background: rgba(10, 10, 10, 0.6);
    border: 1px solid var(--neon-purple);
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.3s ease;
}

.alternative-item:hover {
    border-color: var(--neon-purple-bright);
    box-shadow: 0 0 15px rgba(199, 125, 255, 0.3);
}

.explanation-section {
    margin: 2rem 0;
    padding: 1.5rem;
    background: rgba(10, 10, 10, 0.8);
    border: 1px solid var(--neon-cyan);
    border-radius: 16px;
}

.correct-answer {
    color: var(--neon-cyan);
    font-weight: bold;
    margin-top: 1rem;
    text-shadow: 0 0 10px var(--neon-cyan);
}

.incorrect-answer {
    color: var(--neon-pink);
    font-weight: bold;
    margin-top: 1rem;
    text-shadow: 0 0 10px var(--neon-pink);
}

.question-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 1.5rem;
}

.tag {
    padding: 0.25rem 0.75rem;
    background: rgba(157, 78, 221, 0.2);
    border: 1px solid var(--neon-purple);
    border-radius: 16px;
    font-size: 0.85rem;
    color: var(--neon-purple-light);
}

/* Essay Detail */
.essay-detail {
    max-width: 900px;
    margin: 0 auto;
}

.essay-meta {
    display: flex;
    gap: 1.5rem;
    margin: 1rem 0;
    flex-wrap: wrap;
}

.essay-content {
    margin: 2rem 0;
}

.essay-text {
    background: rgba(10, 10, 10, 0.8);
    border: 1px solid var(--neon-purple);
    border-radius: 16px;
    padding: 1.5rem;
    white-space: pre-wrap;
    line-height: 1.8;
    font-family: inherit;
    color: var(--neon-purple-light);
}

.essay-feedback {
    margin: 2rem 0;
    padding: 1.5rem;
    background: rgba(10, 10, 10, 0.8);
    border: 1px solid var(--neon-cyan);
    border-radius: 16px;
}

.rubric-scores {
    margin: 2rem 0;
}

.rubric-item {
    display: flex;
    justify-content: space-between;
    padding: 0.75rem;
    border-bottom: 1px solid rgba(157, 78, 221, 0.2);
}

.criterion-name {
    color: var(--neon-purple-light);
}

.criterion-score {
    color: var(--gold);
    font-weight: bold;
}

/* Essay Editor */
.essay-editor {
    max-width: 900px;
    margin: 0 auto;
}

.editor-header {
    display: flex;
    gap: 1rem;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
}

.essay-title-input {
    flex: 1;
    min-width: 300px;
}

.neon-select {
    background: rgba(10, 10, 10, 0.8);
    border: 2px solid var(--neon-purple);
    border-radius: 14px;
    color: var(--neon-purple-light);
    padding: 0.75rem 1rem;
    font-family: inherit;
    cursor: pointer;
}

.essay-content-textarea {
    width: 100%;
    min-height: 400px;
    resize: vertical;
    font-family: inherit;
    font-size: 1rem;
    line-height: 1.6;
}

.editor-actions {
    display: flex;
    gap: 1rem;
    margin-top: 1.5rem;
    justify-content: flex-end;
}

/* Empty State */
.empty-state {
    text-align: center;
    padding: 3rem;
    color: var(--neon-purple-light);
    opacity: 0.7;
    font-size: 1.1rem;
}

/* Loading */
.loading {
    text-align: center;
    padding: 3rem;
    color: var(--neon-cyan);
    font-size: 1.2rem;
}

/* Links */
a {
    text-decoration: none;
    color: inherit;
}

/* Trail Cards */
.trails-list {
    margin-top: 2rem;
}

.trail-card {
    margin-bottom: 1.5rem;
    padding: 1.5rem;
    border-radius: 16px;
}

.trail-header {
    display: flex;
    justify-content: space-between;
    align-items: start;
    margin-bottom: 1rem;
}

.trail-title {
    color: var(--neon-purple-light);
    font-size: 1.3rem;
    font-weight: 600;
    margin-bottom: 0.5rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.trail-progress {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 0.25rem;
}

.progress-label {
    font-size: 0.75rem;
    color: var(--neon-purple-light);
    opacity: 0.7;
    text-transform: uppercase;
    letter-spacing: 0.1em;
}

.progress-value {
    font-size: 1.2rem;
    color: var(--neon-cyan);
    font-weight: bold;
    text-shadow: 0 0 10px var(--neon-cyan);
}

.trail-description {
    color: var(--neon-purple-light);
    line-height: 1.6;
    margin-bottom: 1rem;
    opacity: 0.9;
}

.trail-card .neon-progress-bar {
    margin-top: 1rem;
}

/* Responsive Breakpoints */
@media (max-width: 1023px) {
    .two-panel-layout {
        grid-template-columns: 1fr;
        gap: 1.5rem;
    }
    
    .status-bar {
        grid-template-columns: auto 1fr auto;
        padding: 0.75rem 1.5rem;
    }
}

@media (max-width: 767px) {
    .status-bar {
        height: 3.5rem;
        padding: 0.5rem 1rem;
    }
    
    .logo-text {
        font-size: 1.2rem;
    }
    
    .brain-icon {
        font-size: 1.2rem;
    }
    
    .status-time {
        font-size: 1rem;
    }
    
    .version-text {
        display: none;
    }
    
    .page-container {
        padding: 1rem;
        padding-bottom: 6rem;
    }
    
    .page-title {
        font-size: 2rem;
    }
    
    .panel-title {
        font-size: 1.5rem;
    }
    
    .section-title {
        font-size: 1.2rem;
    }
    
    .panel-card {
        padding: 1rem;
    }
    
    .cyber-card {
        padding: 1rem;
    }
    
    .tab-item {
        min-width: 60px;
        padding: 0.5rem 0.75rem;
    }
    
    .tab-label {
        font-size: 0.65rem;
    }
    
    .dashboard-grid {
        gap: 1rem;
    }
}

/* Accessibility - Reduced Motion */
@media (prefers-reduced-motion: reduce) {
    * {
        animation: none !important;
        transition: none !important;
    }
    
    .neon-button:hover,
    .cyber-card:hover {
        transform: none;
    }
    
    .tab-item:hover .tab-icon {
        transform: none;
    }
}

/* Focus States for Keyboard Navigation */
*:focus {
    outline: 2px solid var(--neon-cyan);
    outline-offset: 2px;
    box-shadow: 0 0 10px var(--neon-cyan);
}

button:focus,
input:focus,
select:focus,
textarea:focus,
a:focus {
    outline: 2px solid var(--neon-cyan);
    outline-offset: 2px;
}

/* Print Styles */
@media print {
    .status-bar,
    .tab-bar,
    .brain-watermark {
        display: none;
    }
    
    body {
        background: white;
        color: black;
    }
    
    .cyber-card {
        border: 1px solid black;
        box-shadow: none;
    }
}

/* Import Modal Styles */
.panel-header-with-button {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
}

.import-button {
    padding: 0.6rem 1.5rem;
    background: linear-gradient(135deg, var(--neon-purple) 0%, var(--neon-pink) 100%);
    border: 2px solid var(--neon-purple-bright);
    border-radius: 16px;
    color: var(--white);
    font-size: 0.9rem;
    font-weight: 600;
    font-family: var(--font-heading);
    cursor: pointer;
    transition: all 0.3s ease;
    box-shadow: 0 0 15px rgba(157, 78, 221, 0.4);
    letter-spacing: 0.05em;
}

.import-button:hover {
    background: linear-gradient(135deg, var(--neon-purple-bright) 0%, var(--hot-pink) 100%);
    box-shadow: 0 0 25px rgba(157, 78, 221, 0.8), 0 0 35px rgba(255, 16, 240, 0.5);
    transform: translateY(-2px);
    border-color: var(--neon-pink);
}

.import-button:active {
    transform: translateY(0);
    box-shadow: 0 0 15px rgba(157, 78, 221, 0.6);
}

.import-modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.85);
    backdrop-filter: blur(10px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fade-in 0.3s ease;
}

@keyframes fade-in {
    from {
        opacity: 0;
    }
    to {
        opacity: 1;
    }
}

.import-modal-container {
    background: linear-gradient(135deg, #0a0a0a 0%, #1a1a2e 100%);
    border: 2px solid var(--neon-purple);
    border-radius: 20px;
    box-shadow: 0 0 40px rgba(157, 78, 221, 0.5), 0 0 60px rgba(255, 16, 240, 0.3);
    max-width: 600px;
    width: 90%;
    max-height: 80vh;
    overflow: hidden;
    animation: slide-up 0.3s ease;
}

@keyframes slide-up {
    from {
        transform: translateY(50px);
        opacity: 0;
    }
    to {
        transform: translateY(0);
        opacity: 1;
    }
}

.import-modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 2rem;
    border-bottom: 2px solid var(--neon-purple);
    background: rgba(157, 78, 221, 0.1);
}

.import-modal-title {
    font-family: var(--font-heading);
    font-size: 1.5rem;
    font-weight: 900;
    color: var(--neon-purple-bright);
    text-shadow: 0 0 15px var(--neon-purple);
    letter-spacing: 0.15em;
}

.import-modal-close {
    background: transparent;
    border: none;
    color: var(--neon-pink);
    font-size: 2rem;
    cursor: pointer;
    transition: all 0.3s ease;
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 10px;
}

.import-modal-close:hover {
    color: var(--hot-pink);
    background: rgba(255, 16, 240, 0.2);
    text-shadow: 0 0 15px var(--hot-pink);
    transform: rotate(90deg);
}

.import-modal-content {
    padding: 2rem;
    min-height: 300px;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

.import-options {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.import-description {
    color: var(--neon-purple-light);
    font-size: 1rem;
    margin-bottom: 1rem;
    text-align: center;
}

.import-option-button {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    padding: 1.5rem;
    background: linear-gradient(135deg, rgba(157, 78, 221, 0.1) 0%, rgba(255, 16, 240, 0.05) 100%);
    border: 2px solid var(--neon-purple);
    border-radius: 16px;
    cursor: pointer;
    transition: all 0.3s ease;
    text-align: left;
}

.import-option-button:hover {
    background: linear-gradient(135deg, rgba(157, 78, 221, 0.2) 0%, rgba(255, 16, 240, 0.1) 100%);
    border-color: var(--neon-purple-bright);
    box-shadow: 0 0 20px rgba(157, 78, 221, 0.4);
    transform: translateX(5px);
}

.import-option-icon {
    font-size: 3rem;
    min-width: 60px;
    text-align: center;
    filter: drop-shadow(0 0 10px var(--neon-cyan));
}

.import-option-text h3 {
    color: var(--neon-purple-bright);
    font-family: var(--font-heading);
    font-size: 1.2rem;
    margin-bottom: 0.5rem;
    letter-spacing: 0.05em;
}

.import-option-text p {
    color: var(--light-gray);
    font-size: 0.9rem;
}

.import-processing {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1.5rem;
    min-height: 200px;
}

.import-spinner {
    width: 60px;
    height: 60px;
    border: 4px solid rgba(157, 78, 221, 0.2);
    border-top-color: var(--neon-purple-bright);
    border-radius: 50%;
    animation: spin 1s linear infinite;
    box-shadow: 0 0 20px rgba(157, 78, 221, 0.5);
}

@keyframes spin {
    to {
        transform: rotate(360deg);
    }
}

.import-processing p {
    color: var(--neon-purple-light);
    font-size: 1.1rem;
    font-family: var(--font-heading);
    letter-spacing: 0.05em;
}

.import-feedback {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    min-height: 200px;
    text-align: center;
}

.import-icon {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 3rem;
    font-weight: bold;
}

.import-success .import-icon {
    background: linear-gradient(135deg, rgba(0, 255, 0, 0.2) 0%, rgba(0, 200, 0, 0.1) 100%);
    border: 3px solid #00ff00;
    color: #00ff00;
    box-shadow: 0 0 30px rgba(0, 255, 0, 0.5);
    text-shadow: 0 0 15px #00ff00;
}

.import-error .import-icon {
    background: linear-gradient(135deg, rgba(255, 0, 0, 0.2) 0%, rgba(200, 0, 0, 0.1) 100%);
    border: 3px solid #ff0000;
    color: #ff0000;
    box-shadow: 0 0 30px rgba(255, 0, 0, 0.5);
    text-shadow: 0 0 15px #ff0000;
}

.import-feedback p {
    color: var(--white);
    font-size: 1.1rem;
    max-width: 400px;
    line-height: 1.6;
}

.import-success p {
    color: #00ff00;
    text-shadow: 0 0 10px rgba(0, 255, 0, 0.5);
}

.import-error p {
    color: #ff6666;
    text-shadow: 0 0 10px rgba(255, 0, 0, 0.3);
}
"#;

