# Model Loading Interface Rework

## Overview

Restructure the AI model loading workflow to provide users with a guided configuration experience before model initialization. Currently, the model loading happens automatically at application startup with a blocking loading screen. This design introduces a settings-based approach where users configure their HuggingFace token first, with the option to skip if the model is already cached.

## Background

### Current Implementation

The application currently:
- Initializes AI model automatically on startup via `app.rs`
- Shows a full-screen loading overlay during model download and initialization
- Uses HuggingFace Hub API without explicit token configuration
- Downloads BERTimbau model (420MB) to local cache on first run
- Provides no guidance on HuggingFace token setup or permissions

### Problems

1. **User Experience**: Users encounter a blocking loading screen without context about what's happening or how to configure it
2. **No Configuration Entry Point**: No UI to manage HuggingFace credentials or model settings
3. **Missing Guidance**: Users aren't informed about obtaining HuggingFace tokens or required permissions
4. **Mandatory Loading**: Model loads at startup even when not immediately needed
5. **No Retry Mechanism**: If model loading fails, users must restart the application

## Objectives

1. Move model loading interface from startup screen to settings page
2. Add HuggingFace token configuration UI
3. Provide step-by-step instructions for obtaining a HuggingFace token
4. Include direct links to HuggingFace token creation with required permissions
5. Allow application to start normally and redirect to settings if model is not configured
6. Display model loading progress in settings interface
7. Enable users to retry model loading without restarting the application

## Solution Architecture

### Startup Flow Modification

The application startup logic will be modified to check model and configuration status:

**Startup Decision Tree**:
1. Application starts
2. Check if HuggingFace token is configured
3. Check if model is already cached locally
4. **If both configured and cached**: Application starts normally
5. **If token not configured OR model not cached**: Redirect to Profile/Settings page with AI Configuration tab active
6. Display appropriate messaging based on configuration state

### Settings Interface Structure

The Profile/Settings page will be enhanced with a new section dedicated to AI configuration:

**New Settings Section: AI Model Configuration**

This section will contain:
- HuggingFace token input field (masked/password-style)
- Token validation status indicator
- Model download status
- Progress bar for model loading
- Retry/Cancel buttons for model loading operations
- Link to token acquisition instructions

### Token Configuration Workflow

**Step-by-Step Token Setup Guide**

The interface will display collapsible/expandable instructions:

**Step 1: Create HuggingFace Account**
- Link: https://huggingface.co/join
- Visual indicator: "Create your free HuggingFace account"

**Step 2: Navigate to Token Settings**
- Link: https://huggingface.co/settings/tokens
- Visual indicator: "Go to your token settings"

**Step 3: Create New Token**
- Button/Link: "New token" on HuggingFace
- Token name suggestion: "NeuroNexus-Desktop"
- Required permissions:
  - Read access to repositories
  - Read access to models
- Token type: "Read" (not "Write")

**Step 4: Copy Token**
- Instructions: "Copy the generated token (starts with 'hf_')"
- Warning: "Save this token securely - you won't be able to see it again"

**Step 5: Paste Token in NeuroNexus**
- Input field in the UI
- Validation: Check token format (starts with "hf_")
- Test connection button

### Model Loading Interface

The model loading interface will be integrated within the settings page:

**Components**:

1. **Configuration Status Panel**
   - HuggingFace connection status (Connected/Not Connected)
   - Model download status (Not Downloaded/Downloading/Cached)
   - Last successful model load timestamp
   - Model version information

2. **Progress Display**
   - Multi-stage progress bar showing current operation
   - Progress stages:
     - Validating token
     - Connecting to repository
     - Downloading configuration
     - Downloading tokenizer
     - Downloading model weights
     - Loading model into memory
   - Percentage completion for each stage
   - Current file being downloaded
   - Download speed and estimated time remaining
   - Total downloaded size / Total size

3. **Action Controls**
   - "Download Model" button (enabled when token is valid and model not cached)
   - "Retry Download" button (visible on failure)
   - "Cancel Download" button (visible during download)
   - "Test Model" button (enabled when model is loaded)
   - "Clear Cache" button (removes cached model)

4. **Status Messages**
   - Real-time status updates during operations
   - Success/Error messages with actionable feedback
   - Link to troubleshooting documentation

### Model Status Detection

The system needs to determine model and configuration status at startup:

**Configuration Status Check**:
- Check if HuggingFace token exists in storage
- Validate token format
- Optionally test token connectivity (without blocking)

**Model Cache Status Check**:
- Check HuggingFace cache directory for BERTimbau model
- Verify model files integrity (config.json, tokenizer.json, pytorch_model.bin)
- Determine if model is complete or partially downloaded

**Cache Location by Platform**:
- Linux/macOS: `~/.cache/huggingface/hub/models--neuralmind--bert-base-portuguese-cased/`
- Windows: `%USERPROFILE%\.cache\huggingface\hub\models--neuralmind--bert-base-portuguese-cased\`

### Configuration Storage

HuggingFace token and related settings will be persisted locally:

**Storage Approach**:
- Use platform-specific secure storage where available
- Fallback to encrypted local file
- Store in application configuration directory

**Stored Configuration**:
- HuggingFace token (encrypted at rest)
- Model download preferences
- Last successful connection timestamp
- Model cache path preference (for custom locations)

**Configuration Structure**:
```
AIConfiguration {
    huggingface_token: Optional encrypted string
    auto_load_on_startup: boolean (default: false)
    model_cache_path: Optional custom path
    last_successful_load: Optional timestamp
    model_version: Optional string
}
```

### User Journey Flows

#### First-Time User (No Token, No Model)

1. User launches application
2. Application detects no token configured
3. Application redirects to Profile page, AI Settings tab
4. Display welcome message: "Configure AI Model for Essay Evaluation"
5. Show step-by-step token acquisition guide
6. User follows guide and obtains token
7. User pastes token into input field
8. System validates token format
9. "Download Model" button becomes enabled
10. User clicks "Download Model"
11. Progress bar shows download stages
12. On completion, "Model Ready" status displayed
13. User can navigate to essays and use AI evaluation

#### Returning User (Token Configured, Model Cached)

1. User launches application
2. Application detects token and cached model
3. Application starts normally to Home page
4. AI features available immediately
5. Model loads lazily when first AI evaluation is requested

#### User with Token but No Model Cache

1. User launches application
2. Application detects token but no cached model
3. Application redirects to Profile page, AI Settings tab
4. Display message: "AI Model needs to be downloaded"
5. "Download Model" button already enabled
6. User clicks to download
7. Progress proceeds as normal

#### User After Model Load Failure

1. Model download fails (network error, insufficient space, etc.)
2. Error message displayed with specific failure reason
3. "Retry Download" button appears
4. Optional "Cancel" to skip and use app without AI
5. User can fix issue and retry
6. Or user can navigate away and retry later from settings

### UI Component Integration

#### Profile Page Enhancement

The existing Profile page will be restructured to accommodate AI configuration:

**Layout Modification**:
- Maintain existing two-panel layout
- Add tabbed interface to right panel or create third section
- Tabs: "Preferences" | "AI Configuration" | "Import/Export"

**AI Configuration Tab Contents**:
- Token configuration section (always visible)
- Model status section (always visible)
- Step-by-step guide (collapsible)
- Advanced settings (collapsible)

#### Progress Bar Component Reuse

Leverage existing `NeonProgressBar` component with enhancements:

**Enhanced Features**:
- Multi-stage progress (show which stage is active)
- Sub-progress for individual stages
- Color coding by stage (connecting: cyan, downloading: purple, loading: green)
- Pause/Resume capability (if supported by download library)

#### Status Indicators

Visual status indicators following the cyberpunk theme:

**Connection Status**:
- Not Configured: Gray icon with "Configure Token" text
- Configured but Untested: Yellow icon with "Token Saved" text
- Connected: Green/Cyan pulsing icon with "Connected" text
- Error: Red icon with error message

**Model Status**:
- Not Downloaded: Gray icon with "Download Required" text
- Downloading: Animated purple icon with progress percentage
- Cached: Green icon with "Model Ready" text
- Loading into Memory: Cyan pulsing icon with "Loading..." text
- Loaded: Bright cyan icon with "Active" text
- Error: Red icon with error message

### Navigation and Routing

#### Startup Redirect Logic

When redirect to settings is needed:

**Redirect Behavior**:
- On startup, if configuration needed, navigate to `/perfil?tab=ai-config`
- Preserve any intended destination in URL state
- After configuration complete, optionally redirect to original destination
- Show dismissible banner on other pages: "AI features require configuration"

**User Override**:
- Allow user to dismiss redirect and use app without AI
- Provide persistent navigation hint to complete configuration
- Disable AI-dependent features with clear messaging

### Error Handling and Recovery

#### Token Validation Errors

**Invalid Format**:
- Message: "Token format invalid. HuggingFace tokens start with 'hf_'"
- Action: Clear input, allow retry

**Invalid Credentials**:
- Message: "Token validation failed. Please check your token and try again"
- Link: "Generate a new token"
- Action: Allow retry or token regeneration

**Network Error**:
- Message: "Cannot connect to HuggingFace. Check your internet connection"
- Action: Retry button, offline mode explanation

#### Model Download Errors

**Insufficient Disk Space**:
- Message: "Insufficient disk space. Need 500MB free, have XXX MB"
- Action: Show cache location, suggest cleanup, cancel download

**Network Interruption**:
- Message: "Download interrupted. Progress saved"
- Action: Resume download button (if supported), or retry from checkpoint

**Corrupted Download**:
- Message: "Model files corrupted. Re-downloading affected files"
- Action: Automatic retry of specific files

**Repository Access Denied**:
- Message: "Access denied. Verify your token has 'Read' permissions"
- Link: Token settings page
- Action: Update token

#### Model Loading Errors

**Out of Memory**:
- Message: "Insufficient memory to load model. Close other applications and retry"
- Action: Retry button, link to system requirements

**Incompatible Model Version**:
- Message: "Model version incompatible. Update application or use compatible model"
- Action: Link to updates, clear cache option

### Internationalization

All new UI text must be internationalized using the existing i18n system:

**New Translation Keys Needed** (in `locales/{lang}/profile.ftl`):

```
# AI Configuration Section
profile-ai-title = AI Model Configuration
profile-ai-subtitle = Configure HuggingFace and download AI model
profile-ai-token-label = HuggingFace Token
profile-ai-token-placeholder = Paste your token here (starts with hf_)
profile-ai-token-save = Save Token
profile-ai-token-validate = Validate Token

# Status Messages
profile-ai-status-not-configured = Not Configured
profile-ai-status-configured = Token Configured
profile-ai-status-connected = Connected to HuggingFace
profile-ai-status-model-cached = Model Downloaded
profile-ai-status-model-ready = Model Ready
profile-ai-status-downloading = Downloading Model

# Actions
profile-ai-download-model = Download Model
profile-ai-retry-download = Retry Download
profile-ai-cancel-download = Cancel Download
profile-ai-test-model = Test Model
profile-ai-clear-cache = Clear Model Cache

# Guide
profile-ai-guide-title = How to Get HuggingFace Token
profile-ai-guide-step1 = Step 1: Create HuggingFace Account
profile-ai-guide-step1-desc = Sign up for a free account
profile-ai-guide-step1-link = Create Account
profile-ai-guide-step2 = Step 2: Go to Token Settings
profile-ai-guide-step2-desc = Navigate to your account settings
profile-ai-guide-step2-link = Token Settings
profile-ai-guide-step3 = Step 3: Create New Token
profile-ai-guide-step3-desc = Token type: Read | Required permissions: Read models
profile-ai-guide-step3-name = Token name suggestion: NeuroNexus-Desktop
profile-ai-guide-step4 = Step 4: Copy Token
profile-ai-guide-step4-desc = Copy and save your token securely
profile-ai-guide-step5 = Step 5: Paste in NeuroNexus
profile-ai-guide-step5-desc = Enter your token above and click Save

# Progress Messages
profile-ai-progress-validating = Validating token...
profile-ai-progress-connecting = Connecting to repository...
profile-ai-progress-config = Downloading configuration...
profile-ai-progress-tokenizer = Downloading tokenizer...
profile-ai-progress-weights = Downloading model weights...
profile-ai-progress-loading = Loading model into memory...
profile-ai-progress-complete = Download complete!

# Error Messages
profile-ai-error-invalid-format = Invalid token format
profile-ai-error-invalid-token = Token validation failed
profile-ai-error-network = Network connection error
profile-ai-error-disk-space = Insufficient disk space
profile-ai-error-interrupted = Download interrupted
profile-ai-error-corrupted = Model files corrupted
profile-ai-error-access-denied = Access denied
profile-ai-error-out-of-memory = Insufficient memory
profile-ai-error-incompatible = Incompatible model version

# Info Messages
profile-ai-info-cache-location = Cache location:
profile-ai-info-model-size = Model size: ~420MB
profile-ai-info-first-download = First download may take 5-10 minutes
profile-ai-info-offline-ready = After download, works 100% offline
```

Same translations needed for `pt-BR` and `zh-CN` locales.

### Technical Implementation Considerations

#### State Management

**New Signals/State Required**:
- `ai_token_configured: Signal<bool>`
- `ai_token_value: Signal<String>` (encrypted in memory)
- `ai_model_cached: Signal<bool>`
- `ai_model_loading: Signal<bool>`
- `ai_loading_progress: Signal<f32>`
- `ai_loading_stage: Signal<String>`
- `ai_loading_message: Signal<String>`
- `ai_status: Signal<ModelStatus>` (enum: NotConfigured, Configured, Downloading, Ready, Error)
- `ai_error_message: Signal<Option<String>>`

**ModelStatus Enum**:
```
ModelStatus:
  - NotConfigured
  - TokenSaved
  - Connecting
  - Downloading
  - Loading
  - Ready
  - Error(String)
```

#### Service Layer Modifications

**AIService Enhancements**:

Current `AIService` needs new methods:
- `set_token(token: String)` - Configure HuggingFace token
- `validate_token()` - Test token validity
- `check_model_cache()` - Verify model files exist
- `get_cache_info()` - Return cache location and size
- `clear_cache()` - Remove cached model files
- `cancel_download()` - Abort ongoing download

**Progress Callback Enhancement**:

The existing `ProgressCallback` is good but needs additional context:
- Current stage identifier
- Stage-specific progress (0.0-1.0)
- Overall progress (0.0-1.0)
- File being processed
- Download speed
- Estimated time remaining

**Modified Callback Signature**:
```
ProgressCallback = Arc<dyn Fn(ProgressInfo) + Send + Sync>

ProgressInfo {
    overall_progress: f32
    stage: LoadingStage
    stage_progress: f32
    message: String
    current_file: Option<String>
    download_speed: Option<f64> // bytes per second
    estimated_seconds_remaining: Option<u64>
}

LoadingStage:
  - Validating
  - Connecting
  - DownloadingConfig
  - DownloadingTokenizer
  - DownloadingWeights
  - LoadingModel
  - Complete
```

#### HuggingFace API Integration

**Token Configuration**:

The `hf_hub` crate supports token authentication. Modify `AIService::initialize_with_progress`:

- Accept token as parameter or read from configuration
- Create `Api` instance with token: `Api::new()?.with_token(token)`
- Handle authentication errors gracefully

**Offline Fallback**:

When token not configured but model is cached:
- Attempt to load from cache without API call
- Only require token for downloading new model

#### Configuration Persistence

**Storage Location**:

Platform-specific configuration paths:
- Linux: `~/.config/neuronexus/ai_config.json`
- macOS: `~/Library/Application Support/NeuroNexus/ai_config.json`
- Windows: `%APPDATA%\NeuroNexus\ai_config.json`

**Encryption**:

Token should be encrypted at rest:
- Use platform keychain where available (keyring crate)
- Fallback to application-level encryption
- Never log or expose token in clear text

**Configuration Schema**:
```
{
  "huggingface_token_encrypted": "base64_encrypted_string",
  "auto_load_on_startup": false,
  "model_cache_path": null,
  "last_successful_load": "2024-01-15T10:30:00Z",
  "model_version": "neuralmind/bert-base-portuguese-cased",
  "download_preferences": {
    "resume_on_interrupt": true,
    "verify_integrity": true
  }
}
```

#### App.rs Modification

**Remove Automatic Loading**:

Current `app.rs` has a `use_effect` that initializes AI on startup. This needs to be removed.

**Add Configuration Check**:

Replace automatic initialization with configuration check:
- Check if token configured
- Check if model cached
- If either missing, navigate to settings
- If both present, defer model loading until first use

**Lazy Loading**:

Model should load on-demand:
- First call to `score_essay()` triggers load
- Show progress indicator in essay detail page during first load
- Cache in memory for subsequent evaluations

### Migration Strategy

For existing users who already have the model cached:

**Graceful Migration**:
1. Detect existing model cache without token configured
2. Allow continued use without token
3. Show non-blocking notification: "Configure HuggingFace token for future updates"
4. Token only required for re-downloading or updating model

**Version Detection**:
- Check if this is first launch of new version
- If existing cache detected, set a flag: `legacy_cache_found: true`
- Suppress mandatory configuration requirement
- Show optional configuration prompt

### Testing Scenarios

The implementation should handle these test cases:

1. **Fresh Install, No Network**
   - Expected: Redirect to settings, show "internet required" message
   - Allow user to continue without AI features

2. **Fresh Install, With Network**
   - Expected: Redirect to settings, show guide
   - User can configure token and download model

3. **Existing Cache, No Token**
   - Expected: App starts normally, AI features work
   - Gentle prompt to configure token for updates

4. **Existing Cache, With Token**
   - Expected: App starts normally, full functionality

5. **Token Configured, No Cache**
   - Expected: Redirect to settings, download button enabled
   - One-click download experience

6. **Download Interruption**
   - Expected: Partial progress saved, resume available
   - Clear error message and retry option

7. **Invalid Token**
   - Expected: Clear validation error
   - Link to token creation guide

8. **Insufficient Disk Space**
   - Expected: Error before download starts
   - Show required/available space

9. **Model Corruption**
   - Expected: Integrity check fails, auto-retry
   - Option to clear cache and re-download

10. **Multiple Launches During Download**
    - Expected: Prevent concurrent downloads
    - Show existing download progress

### Performance Considerations

**Startup Performance**:
- Configuration check must be fast (<100ms)
- Cache detection should not block UI
- Defer heavy operations until settings page is active

**Download Performance**:
- Show accurate progress (not simulated)
- Support download resumption
- Implement retry with exponential backoff

**Memory Management**:
- Model stays in memory once loaded
- Clear from memory when not used for extended period
- Provide manual "unload model" option in advanced settings

**UI Responsiveness**:
- All downloads and loading run in background tasks
- UI remains responsive during operations
- Progress updates at reasonable intervals (every 500ms)

### Security Considerations

**Token Security**:
- Never log token value
- Encrypt token at rest
- Use secure memory for token in transit
- Clear token from memory when not in use
- Provide "delete token" option

**Network Security**:
- All HuggingFace communication over HTTPS
- Validate SSL certificates
- Handle man-in-the-middle attack scenarios

**Cache Security**:
- Model cache stored in user-specific directory
- Verify model integrity before loading
- Detect tampered cache files

### Accessibility

**Keyboard Navigation**:
- All token input and buttons keyboard accessible
- Tab order follows logical flow
- Enter key submits token, Space activates buttons

**Screen Reader Support**:
- Progress announcements for screen readers
- Status changes announced
- Error messages accessible

**Visual Indicators**:
- High contrast status indicators
- Progress bar with text percentage backup
- Color-blind friendly error/success states

### Documentation Updates

**User Documentation** (to be created):
- "Getting Started with AI Features" guide
- HuggingFace token setup tutorial with screenshots
- Troubleshooting common setup issues
- FAQ section

**Developer Documentation**:
- AIService API updates
- Configuration storage specification
- State management patterns
- Testing guide for AI features

## Success Criteria

The implementation will be considered successful when:

1. ✅ First-time users are guided to configure HuggingFace token before model download
2. ✅ Step-by-step instructions are clear and include direct links to HuggingFace
3. ✅ Token permissions are explicitly stated in the guide
4. ✅ Application can start and function without AI if model is not configured
5. ✅ Model loading progress is visible and accurate in settings interface
6. ✅ Users can retry failed downloads without restarting the application
7. ✅ Existing users with cached models experience seamless migration
8. ✅ All new UI is internationalized for en-US, pt-BR, and zh-CN
9. ✅ Token is stored securely with encryption
10. ✅ Download can be resumed after interruption
11. ✅ Settings page provides clear status of token and model configuration
12. ✅ Error messages are actionable and user-friendly

## Future Enhancements

Potential improvements beyond this design:

- **Model Selection**: Allow users to choose between different models (size/performance trade-offs)
- **Automatic Updates**: Check for model updates and notify users
- **Offline Package**: Provide downloadable bundle with pre-cached model
- **Advanced Settings**: Custom cache location, model quantization options
- **Usage Statistics**: Show AI usage metrics (evaluations performed, cache size, etc.)
- **Multi-Model Support**: Download and switch between multiple AI models
- **Cloud Sync**: Sync configuration across devices

## Dependencies

**New Dependencies Required**:
- `keyring` crate (for secure token storage)
- Enhanced `hf_hub` usage (token authentication, download resumption)

**Modified Components**:
- `/crates/services/src/ai.rs` - AIService with token support
- `/crates/app/src/app.rs` - Remove auto-initialization
- `/crates/app/src/pages/profile.rs` - Add AI configuration tab
- `/crates/app/src/context.rs` - Add configuration state
- `/crates/app/src/components/mod.rs` - Potential new components for progress display

**New Components** (to be created):
- `AIConfigPanel` component (in `/crates/app/src/components/`)
- `TokenGuide` component (in `/crates/app/src/components/`)
- `ModelStatusIndicator` component (in `/crates/app/src/components/`)
- `AIConfiguration` service (in `/crates/services/src/`)

**Translation Files**:
- `/locales/en-US/profile.ftl` - Additions
- `/locales/pt-BR/profile.ftl` - Additions
- `/locales/zh-CN/profile.ftl` - Additions

## Implementation Phases

### Phase 1: Configuration Storage and Token Management
- Create AIConfiguration service
- Implement secure token storage
- Add token validation logic
- Modify AIService to accept and use token

### Phase 2: Settings UI Development
- Create AI configuration tab in Profile page
- Build token input component
- Implement step-by-step guide UI
- Add status indicators

### Phase 3: Model Management
- Implement cache detection logic
- Add model download controls
- Integrate progress display
- Handle download errors and retry logic

### Phase 4: Startup Flow Modification
- Remove auto-initialization from app.rs
- Add configuration check at startup
- Implement redirect logic
- Add lazy loading for AI features

### Phase 5: Internationalization and Polish
- Add all translation keys
- Translate to pt-BR and zh-CN
- Test all error scenarios
- Refine UI based on feedback

### Phase 6: Migration and Testing
- Implement migration for existing users
- Comprehensive testing of all scenarios
- Performance optimization
- Documentation updates
