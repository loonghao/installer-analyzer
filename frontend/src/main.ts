// Types for the analysis data
interface AnalysisData {
  metadata: {
    filename: string;
    file_size: number;
    format: string;
    version?: string;
    publisher?: string;
    description?: string;
    [key: string]: any;
  };
  files: Array<{
    path: string;
    size: number;
    type?: string;
    is_directory?: boolean;
    icon_class?: string;
  }>;
  registry_operations?: Array<{
    operation: string;
    key: string;
    value?: string;
  }>;
}

interface FileNode {
  name: string;
  path: string;
  size: number;
  is_directory: boolean;
  icon_class: string;
  children?: FileNode[];
}

// Global variable to store analysis data
let analysisData: AnalysisData | null = null;

// Initialize the application
function init() {
  // Try to get data from global variable (injected by Rust)
  if (typeof (window as any).ANALYSIS_DATA !== 'undefined') {
    analysisData = (window as any).ANALYSIS_DATA;
    renderReport();
  } else {
    // Fallback: try to load from a JSON file or show placeholder
    loadSampleData();
  }

  // Setup event listeners
  setupEventListeners();

  // Set generated time
  const now = new Date();
  const timeElement = document.getElementById('generated-time');
  if (timeElement) {
    timeElement.textContent = now.toLocaleString();
  }
}

// Load sample data for development
function loadSampleData() {
  analysisData = {
    metadata: {
      filename: "sample-installer.msi",
      file_size: 1024000,
      format: "MSI",
      version: "1.0.0",
      publisher: "Sample Publisher",
      description: "Sample installer package"
    },
    files: [
      { path: "Program Files", size: 0, type: "folder", is_directory: true, icon_class: "fas fa-folder" },
      { path: "Program Files/App", size: 0, type: "folder", is_directory: true, icon_class: "fas fa-folder" },
      { path: "Program Files/App/app.exe", size: 512000, type: "executable", is_directory: false, icon_class: "fas fa-cog" },
      { path: "Program Files/App/config.ini", size: 1024, type: "config", is_directory: false, icon_class: "fas fa-file-alt" },
      { path: "Program Files/App/readme.txt", size: 2048, type: "text", is_directory: false, icon_class: "fas fa-file-text" },
      { path: "System32", size: 0, type: "folder", is_directory: true, icon_class: "fas fa-folder" },
      { path: "System32/driver.sys", size: 32768, type: "driver", is_directory: false, icon_class: "fas fa-microchip" }
    ],
    registry_operations: [
      { operation: "CREATE", key: "HKLM\\Software\\App", value: "InstallPath" },
      { operation: "SET", key: "HKLM\\Software\\App\\Version", value: "1.0.0" }
    ]
  };
  renderReport();
}

// Render the complete report
function renderReport() {
  if (!analysisData) return;

  renderBasicInfo();
  renderSummary();
  renderFileTree();
  renderRegistryOperations();
}

// Render basic information section
function renderBasicInfo() {
  if (!analysisData) return;

  const metadata = analysisData.metadata;

  // Update header
  const productNameElement = document.getElementById('product-name');
  if (productNameElement) {
    productNameElement.textContent = metadata.filename || 'Package Analysis';
  }

  // Update metadata fields
  updateElementText('filename', metadata.filename);
  updateElementText('version', metadata.version || 'N/A');
  updateElementText('publisher', metadata.publisher || 'N/A');
  updateElementText('format', metadata.format);
  updateElementText('file-size', formatFileSize(metadata.file_size));
  updateElementText('description', metadata.description || 'N/A');
}

// Render summary section
function renderSummary() {
  if (!analysisData) return;

  const files = analysisData.files || [];
  const registryOps = analysisData.registry_operations || [];

  const totalFiles = files.filter(f => !f.is_directory).length;
  const executables = files.filter(f => f.type === 'executable').length;
  const totalSize = files.reduce((sum, f) => sum + f.size, 0);

  updateElementText('total-files', totalFiles.toString());
  updateElementText('registry-ops', registryOps.length.toString());
  updateElementText('executables', executables.toString());
  updateElementText('total-size', formatFileSize(totalSize));
  updateElementText('file-count', totalFiles.toString());
  updateElementText('file-count-display', `${totalFiles} files`);
}

// Render file tree section (macOS Finder style)
function renderFileTree() {
  const finderContainer = document.getElementById('finderContainer');
  if (!finderContainer || !analysisData) return;

  const files = analysisData.files || [];
  const fileTree = buildFileTree(files);

  // Create initial column with root items
  const rootColumn = createFinderColumn(fileTree, []);
  finderContainer.innerHTML = '';
  finderContainer.appendChild(rootColumn);
}

// Build hierarchical file tree from flat file list
function buildFileTree(files: any[]): FileNode[] {
  const tree: FileNode[] = [];
  const pathMap = new Map<string, FileNode>();

  // Sort files to ensure directories come before their contents
  files.sort((a, b) => {
    const aDepth = a.path.split('/').length;
    const bDepth = b.path.split('/').length;
    if (aDepth !== bDepth) return aDepth - bDepth;
    return a.path.localeCompare(b.path);
  });

  files.forEach(file => {
    const parts = file.path.split('/');
    const name = parts[parts.length - 1];

    const node: FileNode = {
      name,
      path: file.path,
      size: file.size,
      is_directory: file.is_directory || false,
      icon_class: file.icon_class || getFileIcon(file.path, file.is_directory),
      children: file.is_directory ? [] : undefined
    };

    pathMap.set(file.path, node);

    if (parts.length === 1) {
      // Root level item
      tree.push(node);
    } else {
      // Find parent
      const parentPath = parts.slice(0, -1).join('/');
      const parent = pathMap.get(parentPath);
      if (parent && parent.children) {
        parent.children.push(node);
      }
    }
  });

  // Calculate directory sizes by summing children
  calculateDirectorySizes(tree);

  return tree;
}

// Calculate directory sizes recursively
function calculateDirectorySizes(nodes: FileNode[]): void {
  nodes.forEach(node => {
    if (node.is_directory && node.children) {
      // First calculate sizes for child directories
      calculateDirectorySizes(node.children);

      // Then sum up all children sizes
      node.size = node.children.reduce((total, child) => total + child.size, 0);
    }
  });
}

// Render registry operations section
function renderRegistryOperations() {
  if (!analysisData) return;

  const operations = analysisData.registry_operations || [];
  const registrySection = document.getElementById('registry-section');
  const registryTableBody = document.getElementById('registry-table-body');
  const registryCount = document.getElementById('registry-count');

  if (operations.length === 0) {
    if (registrySection) {
      registrySection.style.display = 'none';
    }
    return;
  }

  // Show registry section
  if (registrySection) {
    registrySection.style.display = 'block';
  }

  // Update count
  if (registryCount) {
    registryCount.textContent = operations.length.toString();
  }

  // Render table rows
  if (registryTableBody) {
    registryTableBody.innerHTML = operations.map(op => `
      <tr>
        <td><span class="badge bg-primary">${escapeHtml(op.operation)}</span></td>
        <td><code style="font-size: 0.85rem; word-break: break-all;">${escapeHtml(op.key)}</code></td>
        <td style="max-width: 200px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;" title="${op.value ? escapeHtml(op.value) : '-'}">${op.value ? escapeHtml(op.value) : '-'}</td>
        <td>
          <button class="copy-btn" onclick="copyToClipboard('${escapeHtml(op.key)}')">
            <i class="fas fa-copy"></i>
          </button>
        </td>
      </tr>
    `).join('');
  }
}

// Setup event listeners
function setupEventListeners() {
  const searchInput = document.getElementById('searchInput') as HTMLInputElement;
  if (searchInput) {
    searchInput.addEventListener('input', handleFileSearch);
  }
}

// Handle file search
function handleFileSearch(event: Event) {
  const searchTerm = (event.target as HTMLInputElement).value.toLowerCase();
  const finderItems = document.querySelectorAll('.finder-item');
  let matchCount = 0;

  finderItems.forEach(item => {
    const label = item.querySelector('.finder-label')?.textContent?.toLowerCase() || '';
    const isMatch = label.includes(searchTerm);

    if (searchTerm === '' || isMatch) {
      item.classList.remove('hidden');
      if (isMatch && searchTerm !== '') {
        matchCount++;
        // Highlight matching text
        const labelElement = item.querySelector('.finder-label');
        if (labelElement && searchTerm) {
          const originalText = labelElement.textContent || '';
          const highlightedText = originalText.replace(
            new RegExp(`(${escapeRegex(searchTerm)})`, 'gi'),
            '<mark>$1</mark>'
          );
          labelElement.innerHTML = highlightedText;
        }
      }
    } else {
      item.classList.add('hidden');
    }
  });

  // Update search results info
  const searchResults = document.getElementById('searchResults');
  if (searchResults) {
    if (searchTerm) {
      searchResults.textContent = `${matchCount} matches found`;
    } else {
      const totalFiles = document.querySelectorAll('.finder-item').length;
      searchResults.textContent = `${totalFiles} files`;
    }
  }
}

// Escape regex special characters
function escapeRegex(string: string): string {
  return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

// Create a Finder column
function createFinderColumn(items: FileNode[], path: string[]): HTMLElement {
  const column = document.createElement('div');
  column.className = 'finder-column';

  items.forEach(item => {
    const itemElement = document.createElement('div');
    itemElement.className = 'finder-item';
    if (item.children && item.children.length > 0) {
      itemElement.classList.add('has-children');
    }

    itemElement.innerHTML = `
      <span class="finder-icon"><i class="${item.icon_class}"></i></span>
      <span class="finder-label" title="${escapeHtml(item.path)}">${escapeHtml(item.name)}</span>
      <span class="finder-size">${formatFileSize(item.size)}</span>
    `;

    itemElement.addEventListener('click', () => {
      // Clear selection in this column
      column.querySelectorAll('.finder-item.selected').forEach(el => {
        el.classList.remove('selected');
      });

      // Select this item
      itemElement.classList.add('selected');

      // Remove subsequent columns
      const container = column.parentElement!;
      let nextSibling = column.nextElementSibling;
      while (nextSibling) {
        const toRemove = nextSibling;
        nextSibling = nextSibling.nextElementSibling;
        container.removeChild(toRemove);
      }

      // Add new column if this item has children
      if (item.children && item.children.length > 0) {
        const newPath = [...path, item.name];
        const newColumn = createFinderColumn(item.children, newPath);
        container.appendChild(newColumn);
      }
    });

    // Add right-click context menu for copying path
    itemElement.addEventListener('contextmenu', (e) => {
      e.preventDefault();
      copyToClipboard(item.path);
    });

    // Add double-click to copy path
    itemElement.addEventListener('dblclick', (e) => {
      e.preventDefault();
      copyToClipboard(item.path);
    });

    column.appendChild(itemElement);
  });

  return column;
}

// Get appropriate icon for file type
function getFileIcon(path: string, isDirectory: boolean): string {
  if (isDirectory) return 'fas fa-folder';

  const ext = path.split('.').pop()?.toLowerCase();
  switch (ext) {
    case 'exe': case 'msi': case 'dmg': return 'fas fa-cog';
    case 'dll': case 'so': case 'dylib': return 'fas fa-puzzle-piece';
    case 'txt': case 'md': case 'readme': return 'fas fa-file-alt';
    case 'pdf': return 'fas fa-file-pdf';
    case 'jpg': case 'jpeg': case 'png': case 'gif': return 'fas fa-file-image';
    case 'mp3': case 'wav': case 'ogg': return 'fas fa-file-audio';
    case 'mp4': case 'avi': case 'mov': return 'fas fa-file-video';
    case 'zip': case 'rar': case '7z': return 'fas fa-file-archive';
    case 'js': case 'ts': case 'py': case 'java': case 'cpp': return 'fas fa-file-code';
    default: return 'fas fa-file';
  }
}

// Update element text content
function updateElementText(id: string, text: string) {
  const element = document.getElementById(id);
  if (element) {
    element.textContent = text;
  }
}

// Escape HTML to prevent XSS
function escapeHtml(text: string): string {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

// Utility functions
function formatFileSize(bytes: number): string {
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  if (bytes === 0) return '0 Bytes';
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return Math.round(bytes / Math.pow(1024, i) * 100) / 100 + ' ' + sizes[i];
}

function truncatePath(path: string, maxLevels: number = 5): string {
  const parts = path.split(/[/\\]/);
  if (parts.length <= maxLevels) return path;

  const filename = parts[parts.length - 1];
  return `.../${filename}`;
}

function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text).then(() => {
    showToast('Copied to clipboard!', 'success');
  }).catch(err => {
    console.error('Failed to copy:', err);
    showToast('Failed to copy to clipboard', 'error');
  });
}

// Show toast notification
function showToast(message: string, type: 'success' | 'error' = 'success') {
  // Remove existing toasts
  const existingToasts = document.querySelectorAll('.toast-notification');
  existingToasts.forEach(toast => toast.remove());

  const toast = document.createElement('div');
  toast.className = `toast-notification toast-${type}`;
  toast.textContent = message;

  // Add styles
  Object.assign(toast.style, {
    position: 'fixed',
    top: '20px',
    right: '20px',
    padding: '12px 20px',
    borderRadius: '6px',
    color: 'white',
    fontWeight: '500',
    fontSize: '14px',
    zIndex: '9999',
    opacity: '0',
    transform: 'translateY(-20px)',
    transition: 'all 0.3s ease',
    backgroundColor: type === 'success' ? '#059669' : '#dc2626',
    boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)'
  });

  document.body.appendChild(toast);

  // Animate in
  setTimeout(() => {
    toast.style.opacity = '1';
    toast.style.transform = 'translateY(0)';
  }, 10);

  // Remove after 3 seconds
  setTimeout(() => {
    toast.style.opacity = '0';
    toast.style.transform = 'translateY(-20px)';
    setTimeout(() => toast.remove(), 300);
  }, 3000);
}

// Make copyToClipboard available globally
(window as any).copyToClipboard = copyToClipboard;

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', init);
