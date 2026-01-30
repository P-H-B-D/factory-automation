import init, { GameState, FurnaceData, ChestData, DrillData, Item, DroppedItem } from './pkg/wasm_exploration.js';

// Type definitions for our game state
let gameState: GameState | null = null;
let canvas: HTMLCanvasElement | null = null;
let ctx: CanvasRenderingContext2D | null = null;
const keys: Record<string, boolean> = {};
let cursorTileX: number | null = null;
let cursorTileY: number | null = null;
let furnaceImage: HTMLImageElement | null = null;

// Constants
let VIEWPORT_WIDTH = 600;
let VIEWPORT_HEIGHT = 400;
let CONSOLE_WIDTH = 250;
let HELP_BOX_HEIGHT = 120; // Height for help box below viewport
const TILE_SIZE = 20; // 1 game tile = 20 pixels

// Function to update viewport dimensions based on window size
function updateViewportDimensions(): void {
    VIEWPORT_WIDTH = window.innerWidth - CONSOLE_WIDTH - 4; // Subtract border
    VIEWPORT_HEIGHT = window.innerHeight - HELP_BOX_HEIGHT - 4; // Subtract border
}

async function run(): Promise<void> {
    // Load font first
    const font = new FontFace('Fusion Pixel', 'url(fusion-pixel.ttf)');
    await font.load();
    // @ts-ignore - FontFaceSet.add is not in TypeScript definitions but exists in browsers
    document.fonts.add(font);
    
    // Initialize WASM
    await init();
    
    // Load furnace image
    furnaceImage = new Image();
    furnaceImage.src = 'assets/furnace.png';
    await new Promise<void>((resolve, reject) => {
        if (furnaceImage) {
            furnaceImage.onload = () => resolve();
            furnaceImage.onerror = () => reject(new Error('Failed to load furnace image'));
        } else {
            reject(new Error('Failed to create image'));
        }
    });
    
    // Create game state (this will generate a new map)
    gameState = new GameState();
    
    // Setup canvas
    canvas = document.getElementById('game-canvas') as HTMLCanvasElement;
    if (!canvas) {
        throw new Error('Canvas element not found');
    }
    
    const context = canvas.getContext('2d');
    if (!context) {
        throw new Error('Could not get 2D context');
    }
    ctx = context;
    
    // Update viewport dimensions and set canvas size
    updateViewportDimensions();
    canvas.width = VIEWPORT_WIDTH + CONSOLE_WIDTH;
    canvas.height = VIEWPORT_HEIGHT + HELP_BOX_HEIGHT;
    
    // Handle window resize
    window.addEventListener('resize', () => {
        updateViewportDimensions();
        if (canvas) {
            canvas.width = VIEWPORT_WIDTH + CONSOLE_WIDTH;
            canvas.height = VIEWPORT_HEIGHT + HELP_BOX_HEIGHT;
        }
    });
    
    // Setup mouse tracking for cursor
    canvas.addEventListener('mousemove', (e: MouseEvent) => {
        if (!canvas) return;
        
        const rect = canvas.getBoundingClientRect();
        const mouseX = e.clientX - rect.left;
        const mouseY = e.clientY - rect.top;
        
        // Only track mouse if it's within the game viewport (not the console)
        if (mouseX < VIEWPORT_WIDTH && gameState) {
            // Calculate camera position (same as in render function)
            const playerTileX = gameState.player_x;
            const playerTileY = gameState.player_y;
            const playerPixelX = playerTileX * TILE_SIZE;
            const playerPixelY = playerTileY * TILE_SIZE;
            
            const cameraX = playerPixelX - VIEWPORT_WIDTH / 2 + TILE_SIZE / 2;
            const cameraY = playerPixelY - VIEWPORT_HEIGHT / 2 + TILE_SIZE / 2;
            
            // Convert mouse position to world coordinates (accounting for camera translation)
            // ctx.translate(-cameraX, -cameraY) means screen (0,0) maps to world (cameraX, cameraY)
            const worldPixelX = mouseX + cameraX;
            const worldPixelY = mouseY + cameraY;
            
            // Convert to tile coordinates
            const worldTileX = Math.floor(worldPixelX / TILE_SIZE);
            const worldTileY = Math.floor(worldPixelY / TILE_SIZE);
            
            cursorTileX = worldTileX;
            cursorTileY = worldTileY;
        } else {
            cursorTileX = null;
            cursorTileY = null;
        }
    });
    
    canvas.addEventListener('mouseleave', () => {
        cursorTileX = null;
        cursorTileY = null;
    });
    
    // Setup keyboard listeners
    window.addEventListener('keydown', (e: KeyboardEvent) => {
        const key = e.key.toLowerCase();
        if (['w', 'a', 's', 'd', 'm', 'f', 'h', '[', ']', 'b', 'j', 'r', 'p', 'c', 't'].includes(key)) {
            keys[key] = true;
            e.preventDefault();
        }
        // Handle space key separately
        if (e.key === ' ') {
            keys[' '] = true;
            e.preventDefault();
        }
        // Handle delete key separately
        if (e.key === 'Delete' || e.key === 'Backspace') {
            keys['delete'] = true;
            e.preventDefault();
        }
    });
    
    window.addEventListener('keyup', (e: KeyboardEvent) => {
        const key = e.key.toLowerCase();
        if (['w', 'a', 's', 'd'].includes(key)) {
            keys[key] = false;
            e.preventDefault();
        }
        // Handle space key separately
        if (e.key === ' ') {
            keys[' '] = false;
            e.preventDefault();
        }
    });
    
    // Game tick function - runs at 60 ticks per second
    function gameTick(): void {
        if (!gameState || !ctx) return;
        
        // Update game state
        const keysObj: Record<string, boolean> = {
            'w': keys['w'] || false,
            'a': keys['a'] || false,
            's': keys['s'] || false,
            'd': keys['d'] || false,
            'm': keys['m'] || false,
            'f': keys['f'] || false,
            ' ': keys[' '] || false,
            'h': keys['h'] || false,
            '[': keys['['] || false,
            ']': keys[']'] || false,
            'b': keys['b'] || false,
            'j': keys['j'] || false,
            'r': keys['r'] || false,
            'p': keys['p'] || false,
            'c': keys['c'] || false,
            't': keys['t'] || false,
            'delete': keys['delete'] || false,
        };
        
        // Pass cursor coordinates (convert to u32 or null)
        const cursorX = cursorTileX !== null && cursorTileX >= 0 ? cursorTileX : null;
        const cursorY = cursorTileY !== null && cursorTileY >= 0 ? cursorTileY : null;
        
        gameState.next_step(keysObj, cursorX, cursorY);
        
        // Clear one-time action keys after processing
        keys['m'] = false;
        keys['f'] = false;
        keys[' '] = false;
        keys['h'] = false;
        keys['['] = false;
        keys[']'] = false;
        keys['b'] = false;
        keys['j'] = false;
        keys['r'] = false;
        keys['p'] = false;
        keys['c'] = false;
        keys['t'] = false;
        keys['delete'] = false;
    }
    
    // Render loop - runs at display refresh rate for smooth rendering
    function renderLoop(): void {
        if (!gameState || !ctx) return;
        
        // Render
        render();
        
        // Continue render loop
        requestAnimationFrame(renderLoop);
    }
    
    // Start game tick loop at 60 TPS (1000ms / 60 = ~16.67ms per tick)
    setInterval(gameTick, 1000 / 60);
    
    // Start render loop
    renderLoop();
}

function render(): void {
    if (!gameState || !ctx || !canvas) return;
    
    // Calculate camera position to center on player (in tile coordinates)
    const playerTileX = gameState.player_x;
    const playerTileY = gameState.player_y;
    
    // Convert player position to pixels
    const playerPixelX = playerTileX * TILE_SIZE;
    const playerPixelY = playerTileY * TILE_SIZE;
    
    // Calculate camera offset (same as in mouse tracking)
    const viewportCenterTileX = Math.floor(VIEWPORT_WIDTH / 2 / TILE_SIZE);
    const viewportCenterTileY = Math.floor(VIEWPORT_HEIGHT / 2 / TILE_SIZE);
    
    const cameraOffsetX = playerTileX - viewportCenterTileX;
    const cameraOffsetY = playerTileY - viewportCenterTileY;
    
    // Camera position (top-left of viewport in pixel coordinates)
    // Always center on player, even if it shows empty space
    const cameraX = playerPixelX - VIEWPORT_WIDTH / 2 + TILE_SIZE / 2;
    const cameraY = playerPixelY - VIEWPORT_HEIGHT / 2 + TILE_SIZE / 2;
    
    // Clear canvas
    ctx.fillStyle = '#2d5016'; // Dark green background
    ctx.fillRect(0, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT);
    
    // Clear console area
    ctx.fillStyle = '#1a1a1a'; // Dark background for console
    ctx.fillRect(VIEWPORT_WIDTH, 0, CONSOLE_WIDTH, VIEWPORT_HEIGHT);
    
    // Save context and translate for camera
    ctx.save();
    ctx.translate(-cameraX, -cameraY);
    
    // Draw the entire map background (in case camera shows outside map)
    const mapWidthPixels = gameState.map_width * TILE_SIZE;
    const mapHeightPixels = gameState.map_height * TILE_SIZE;
    ctx.fillStyle = '#2d5016'; // Dark green background
    ctx.fillRect(0, 0, mapWidthPixels, mapHeightPixels);
    
    // Draw water patches (each patch is 1x1 to 5x5 tiles)
    ctx.fillStyle = '#1e3a8a'; // Blue water
    const waterPatches = gameState.water_patches();
    if (waterPatches && waterPatches.length > 0) {
        for (let i = 0; i < waterPatches.length; i++) {
            const patch = waterPatches[i];
            // Convert tile coordinates to pixel coordinates
            const patchX = patch.x * TILE_SIZE;
            const patchY = patch.y * TILE_SIZE;
            const patchWidth = patch.width * TILE_SIZE;
            const patchHeight = patch.height * TILE_SIZE;
            ctx.fillRect(patchX, patchY, patchWidth, patchHeight);
        }
    }
    
    // Draw resources (iron ore, copper, stone, coal)
    const resources = gameState.resources();
    if (resources && resources.length > 0) {
        for (let i = 0; i < resources.length; i++) {
            const resource = resources[i];
            const resourceX = resource.x * TILE_SIZE;
            const resourceY = resource.y * TILE_SIZE;
            const resourceType = resource.resource_type_value();
            
            // Set color based on resource type
            if (resourceType === 0) { // IronOre
                ctx.fillStyle = '#78716c'; // Gray/brown
            } else if (resourceType === 1) { // Copper
                ctx.fillStyle = '#b87333'; // Copper color
            } else if (resourceType === 2) { // Stone
                ctx.fillStyle = '#6b7280'; // Gray stone
            } else if (resourceType === 3) { // Coal
                ctx.fillStyle = '#1f2937'; // Dark gray/black
            }
            
            ctx.fillRect(resourceX, resourceY, TILE_SIZE, TILE_SIZE);
            
            // Draw a small highlight
            ctx.fillStyle = 'rgba(255, 255, 255, 0.2)';
            ctx.fillRect(resourceX + 2, resourceY + 2, TILE_SIZE - 4, TILE_SIZE - 4);
        }
    }
    
    // Draw placeable objects (furnaces, etc.)
    const placeableObjects = gameState.placeable_objects();
    if (placeableObjects && placeableObjects.length > 0) {
        for (let i = 0; i < placeableObjects.length; i++) {
            const obj = placeableObjects[i];
            const objX = obj.x * TILE_SIZE;
            const objY = obj.y * TILE_SIZE;
            const objType = obj.placeable_type_value();
            
            // Set color based on object type
            if (objType === 0) { // Furnace
                // Draw furnace image, resized to TILE_SIZE
                if (furnaceImage && furnaceImage.complete) {
                    // Disable image smoothing for crisp pixel art
                    const oldSmoothing = ctx.imageSmoothingEnabled;
                    ctx.imageSmoothingEnabled = false;
                    ctx.drawImage(furnaceImage, objX, objY, TILE_SIZE, TILE_SIZE);
                    // Restore previous smoothing setting
                    ctx.imageSmoothingEnabled = oldSmoothing;
                } else {
                    // Fallback to colored rectangle if image not loaded
                    ctx.fillStyle = '#92400e'; // Brown/dark orange for furnace
                    ctx.fillRect(objX, objY, TILE_SIZE, TILE_SIZE);
                    
                    // Draw furnace details (simple representation)
                    ctx.fillStyle = '#1f2937'; // Dark gray for top
                    ctx.fillRect(objX + 2, objY + 2, TILE_SIZE - 4, 4);
                    
                    // Draw fire/glow effect
                    ctx.fillStyle = '#dc2626'; // Red for fire
                    ctx.fillRect(objX + 6, objY + TILE_SIZE - 8, 8, 4);
                }
                
                // Draw floating text box above furnace showing inventory (only when hovering)
                const isHovering = cursorTileX === obj.x && cursorTileY === obj.y;
                const furnaceData: FurnaceData | undefined = gameState.get_furnace_data(obj.x, obj.y);
                if (furnaceData && isHovering) {
                    // TypeScript knows these are properties, not methods!
                    const coalCount = furnaceData.coal_count;
                    const ironOreCount = furnaceData.iron_ore_count;
                    const ironPlateCount = furnaceData.iron_plate_count;
                    const copperCount = furnaceData.copper_count;
                    const copperPlateCount = furnaceData.copper_plate_count;
                    const processingTicks = furnaceData.processing_ticks_remaining;
                    
                    // Only show items with count > 0
                    const lines: string[] = [];
                    if (coalCount > 0) lines.push(`Coal: ${coalCount}`);
                    if (ironOreCount > 0) lines.push(`Iron: ${ironOreCount}`);
                    if (ironPlateCount > 0) lines.push(`IronP: ${ironPlateCount}`);
                    if (copperCount > 0) lines.push(`Copper: ${copperCount}`);
                    if (copperPlateCount > 0) lines.push(`CopperP: ${copperPlateCount}`);
                    
                    // Show processing progress
                    if (processingTicks > 0) {
                        const progress = ((60 - processingTicks) / 60 * 100).toFixed(0);
                        lines.push(`Processing: ${progress}%`);
                    }
                    
                    if (lines.length === 0) {
                        lines.push('Empty');
                    }
                    
                    const textBoxX = objX;
                    const textBoxY = objY - (lines.length * 12 + 8); // Above the furnace, dynamic height
                    const textBoxWidth = 80;
                    const textBoxHeight = lines.length * 12 + 6;
                    
                    // Draw semi-transparent background
                    ctx.fillStyle = 'rgba(0, 0, 0, 0.6)';
                    ctx.fillRect(textBoxX - (textBoxWidth - TILE_SIZE) / 2, textBoxY, textBoxWidth, textBoxHeight);
                    
                    // Draw border
                    ctx.strokeStyle = 'rgba(255, 255, 255, 0.5)';
                    ctx.lineWidth = 1;
                    ctx.strokeRect(textBoxX - (textBoxWidth - TILE_SIZE) / 2, textBoxY, textBoxWidth, textBoxHeight);
                    
                    // Draw text
                    ctx.fillStyle = 'rgba(255, 255, 255, 0.8)';
                    ctx.font = '10px "Fusion Pixel"';
                    
                    let textY = textBoxY + 12;
                    for (const line of lines) {
                        ctx.fillText(line, textBoxX - (textBoxWidth - TILE_SIZE) / 2 + 5, textY);
                        textY += 12;
                    }
                }
            } else if (objType === 1) { // Belt
                // Draw belt as a gray rectangle
                ctx.fillStyle = '#4b5563'; // Gray belt
                ctx.fillRect(objX, objY, TILE_SIZE, TILE_SIZE);
                
                // Get belt direction
                const direction = obj.direction_value();
                
                // Draw belt pattern (lines perpendicular to direction)
                ctx.fillStyle = '#6b7280'; // Lighter gray
                if (direction === 0 || direction === 1) { // North or South - vertical lines
                    ctx.fillRect(objX + TILE_SIZE / 2 - 1, objY, 2, TILE_SIZE);
                } else { // East or West - horizontal lines
                    ctx.fillRect(objX, objY + TILE_SIZE / 2 - 1, TILE_SIZE, 2);
                }
                
                // Draw arrow in belt direction
                ctx.fillStyle = '#9ca3af'; // Light gray for arrow
                ctx.beginPath();
                if (direction === 0) { // North
                    ctx.moveTo(objX + TILE_SIZE / 2, objY + 4);
                    ctx.lineTo(objX + TILE_SIZE / 2 - 3, objY + 8);
                    ctx.lineTo(objX + TILE_SIZE / 2 + 3, objY + 8);
                } else if (direction === 1) { // South
                    ctx.moveTo(objX + TILE_SIZE / 2, objY + TILE_SIZE - 4);
                    ctx.lineTo(objX + TILE_SIZE / 2 - 3, objY + TILE_SIZE - 8);
                    ctx.lineTo(objX + TILE_SIZE / 2 + 3, objY + TILE_SIZE - 8);
                } else if (direction === 2) { // East
                    ctx.moveTo(objX + TILE_SIZE - 4, objY + TILE_SIZE / 2);
                    ctx.lineTo(objX + TILE_SIZE - 8, objY + TILE_SIZE / 2 - 3);
                    ctx.lineTo(objX + TILE_SIZE - 8, objY + TILE_SIZE / 2 + 3);
                } else { // West
                    ctx.moveTo(objX + 4, objY + TILE_SIZE / 2);
                    ctx.lineTo(objX + 8, objY + TILE_SIZE / 2 - 3);
                    ctx.lineTo(objX + 8, objY + TILE_SIZE / 2 + 3);
                }
                ctx.closePath();
                ctx.fill();
            } else if (objType === 2) { // Arm
                // Draw arm as a blue rectangle
                ctx.fillStyle = '#3b82f6'; // Blue for arm
                ctx.fillRect(objX, objY, TILE_SIZE, TILE_SIZE);
                
                // Get arm direction
                const direction = obj.direction_value();
                
                // Draw arm pattern (mechanical look)
                ctx.fillStyle = '#1e40af'; // Darker blue
                ctx.fillRect(objX + 2, objY + 2, TILE_SIZE - 4, TILE_SIZE - 4);
                
                // Draw direction indicator (arrow pointing in direction)
                ctx.fillStyle = '#60a5fa'; // Light blue for arrow
                ctx.beginPath();
                if (direction === 0) { // North
                    ctx.moveTo(objX + TILE_SIZE / 2, objY + 4);
                    ctx.lineTo(objX + TILE_SIZE / 2 - 3, objY + 8);
                    ctx.lineTo(objX + TILE_SIZE / 2 + 3, objY + 8);
                } else if (direction === 1) { // South
                    ctx.moveTo(objX + TILE_SIZE / 2, objY + TILE_SIZE - 4);
                    ctx.lineTo(objX + TILE_SIZE / 2 - 3, objY + TILE_SIZE - 8);
                    ctx.lineTo(objX + TILE_SIZE / 2 + 3, objY + TILE_SIZE - 8);
                } else if (direction === 2) { // East
                    ctx.moveTo(objX + TILE_SIZE - 4, objY + TILE_SIZE / 2);
                    ctx.lineTo(objX + TILE_SIZE - 8, objY + TILE_SIZE / 2 - 3);
                    ctx.lineTo(objX + TILE_SIZE - 8, objY + TILE_SIZE / 2 + 3);
                } else { // West
                    ctx.moveTo(objX + 4, objY + TILE_SIZE / 2);
                    ctx.lineTo(objX + 8, objY + TILE_SIZE / 2 - 3);
                    ctx.lineTo(objX + 8, objY + TILE_SIZE / 2 + 3);
                }
                ctx.closePath();
                ctx.fill();
            } else if (objType === 3) { // Chest
                // Draw chest as a brown rectangle
                ctx.fillStyle = '#8b4513'; // Brown for chest
                ctx.fillRect(objX, objY, TILE_SIZE, TILE_SIZE);
                
                // Draw chest details
                ctx.fillStyle = '#654321'; // Darker brown for lid
                ctx.fillRect(objX + 2, objY + 2, TILE_SIZE - 4, 4);
                
                // Draw lock/handle
                ctx.fillStyle = '#1f2937'; // Dark gray for lock
                ctx.fillRect(objX + TILE_SIZE / 2 - 2, objY + TILE_SIZE / 2 - 2, 4, 4);
                
                // Draw floating text box above chest showing inventory (only when hovering)
                const isHovering = cursorTileX === obj.x && cursorTileY === obj.y;
                const chestData: ChestData | undefined = gameState.get_chest_data(obj.x, obj.y);
                if (chestData && isHovering) {
                    const items = chestData.get_all_items();
                    
                    // Only show items with count > 0
                    const lines: string[] = [];
                    for (let i = 0; i < items.length; i++) {
                        const item = items[i];
                        const quantity = chestData.get_item_quantity(i);
                        if (quantity > 0) {
                            const itemName = getItemName(item);
                            lines.push(`${itemName}: ${quantity}`);
                        }
                    }
                    
                    if (lines.length === 0) {
                        lines.push('Empty');
                    }
                    
                    const textBoxX = objX;
                    const textBoxY = objY - (lines.length * 12 + 8); // Above the chest, dynamic height
                    const textBoxWidth = 100;
                    const textBoxHeight = lines.length * 12 + 6;
                    
                    // Draw semi-transparent background
                    ctx.fillStyle = 'rgba(0, 0, 0, 0.6)';
                    ctx.fillRect(textBoxX - (textBoxWidth - TILE_SIZE) / 2, textBoxY, textBoxWidth, textBoxHeight);
                    
                    // Draw border
                    ctx.strokeStyle = 'rgba(255, 255, 255, 0.5)';
                    ctx.lineWidth = 1;
                    ctx.strokeRect(textBoxX - (textBoxWidth - TILE_SIZE) / 2, textBoxY, textBoxWidth, textBoxHeight);
                    
                    // Draw text
                    ctx.fillStyle = 'rgba(255, 255, 255, 0.8)';
                    ctx.font = '10px "Fusion Pixel"';
                    
                    let textY = textBoxY + 12;
                    for (const line of lines) {
                        ctx.fillText(line, textBoxX - (textBoxWidth - TILE_SIZE) / 2 + 5, textY);
                        textY += 12;
                    }
                }
            } else if (objType === 4) { // Drill
                // Draw drill smaller so resource underneath is visible
                // Make it 60% of tile size, centered
                const drillSize = TILE_SIZE * 0.6;
                const drillOffset = (TILE_SIZE - drillSize) / 2;
                const drillX = objX + drillOffset;
                const drillY = objY + drillOffset;
                
                // Draw drill as a dark gray rectangle (smaller)
                ctx.fillStyle = '#374151'; // Dark gray for drill
                ctx.fillRect(drillX, drillY, drillSize, drillSize);
                
                // Draw drill bit (rotating part)
                ctx.fillStyle = '#1f2937'; // Darker gray for bit
                ctx.fillRect(drillX + 2, drillY + 2, drillSize - 4, drillSize - 4);
                
                // Draw direction indicator (arrow pointing in direction)
                const direction = obj.direction_value();
                ctx.fillStyle = '#6b7280'; // Medium gray for arrow
                ctx.beginPath();
                if (direction === 0) { // North
                    ctx.moveTo(objX + TILE_SIZE / 2, drillY);
                    ctx.lineTo(objX + TILE_SIZE / 2 - 3, drillY + 4);
                    ctx.lineTo(objX + TILE_SIZE / 2 + 3, drillY + 4);
                } else if (direction === 1) { // South
                    ctx.moveTo(objX + TILE_SIZE / 2, drillY + drillSize);
                    ctx.lineTo(objX + TILE_SIZE / 2 - 3, drillY + drillSize - 4);
                    ctx.lineTo(objX + TILE_SIZE / 2 + 3, drillY + drillSize - 4);
                } else if (direction === 2) { // East
                    ctx.moveTo(drillX + drillSize, objY + TILE_SIZE / 2);
                    ctx.lineTo(drillX + drillSize - 4, objY + TILE_SIZE / 2 - 3);
                    ctx.lineTo(drillX + drillSize - 4, objY + TILE_SIZE / 2 + 3);
                } else { // West
                    ctx.moveTo(drillX, objY + TILE_SIZE / 2);
                    ctx.lineTo(drillX + 4, objY + TILE_SIZE / 2 - 3);
                    ctx.lineTo(drillX + 4, objY + TILE_SIZE / 2 + 3);
                }
                ctx.closePath();
                ctx.fill();
                
                // Draw floating text box above drill showing coal (only when hovering)
                const isHovering = cursorTileX === obj.x && cursorTileY === obj.y;
                const drillData: DrillData | undefined = gameState.get_drill_data(obj.x, obj.y);
                if (drillData && isHovering) {
                    const coalCount = drillData.coal_count;
                    const processingTicks = drillData.processing_ticks_remaining;
                    
                    const lines: string[] = [];
                    if (coalCount > 0) lines.push(`Coal: ${coalCount}`);
                    if (processingTicks > 0) {
                        const progress = ((60 - processingTicks) / 60 * 100).toFixed(0);
                        lines.push(`Processing: ${progress}%`);
                    }
                    if (lines.length === 0) {
                        lines.push('Empty');
                    }
                    
                    const textBoxX = objX;
                    const textBoxY = objY - (lines.length * 12 + 8);
                    const textBoxWidth = 80;
                    const textBoxHeight = lines.length * 12 + 6;
                    
                    ctx.fillStyle = 'rgba(0, 0, 0, 0.6)';
                    ctx.fillRect(textBoxX - (textBoxWidth - TILE_SIZE) / 2, textBoxY, textBoxWidth, textBoxHeight);
                    
                    ctx.strokeStyle = 'rgba(255, 255, 255, 0.5)';
                    ctx.lineWidth = 1;
                    ctx.strokeRect(textBoxX - (textBoxWidth - TILE_SIZE) / 2, textBoxY, textBoxWidth, textBoxHeight);
                    
                    ctx.fillStyle = 'rgba(255, 255, 255, 0.8)';
                    ctx.font = '10px "Fusion Pixel"';
                    
                    let textY = textBoxY + 12;
                    for (const line of lines) {
                        ctx.fillText(line, textBoxX - (textBoxWidth - TILE_SIZE) / 2 + 5, textY);
                        textY += 12;
                    }
                }
            }
        }
    }
    
    // Helper function to get item color
    function getItemColor(item: Item): string {
        if (item === Item.IronOre) return '#78716c'; // Gray/brown
        if (item === Item.Copper) return '#b87333'; // Copper color
        if (item === Item.Stone) return '#6b7280'; // Gray stone
        if (item === Item.Coal) return '#1f2937'; // Dark gray/black
        if (item === Item.Furnace) return '#92400e'; // Brown/dark orange
        if (item === Item.IronPlate) return '#9ca3af'; // Light gray
        if (item === Item.Belt) return '#4b5563'; // Gray belt
        if (item === Item.CopperPlate) return '#f59e0b'; // Amber/orange
        if (item === Item.Arm) return '#3b82f6'; // Blue
        if (item === Item.Chest) return '#8b4513'; // Brown
        if (item === Item.Drill) return '#374151'; // Dark gray
        return '#ffffff'; // Default white
    }
    
    // Helper function to get item name
    function getItemName(item: Item): string {
        if (item === Item.IronOre) return 'Iron Ore';
        if (item === Item.Copper) return 'Copper';
        if (item === Item.Stone) return 'Stone';
        if (item === Item.Coal) return 'Coal';
        if (item === Item.Furnace) return 'Furnace';
        if (item === Item.IronPlate) return 'Iron Plate';
        if (item === Item.Belt) return 'Belt';
        if (item === Item.CopperPlate) return 'Copper Plate';
        if (item === Item.Arm) return 'Arm';
        if (item === Item.Chest) return 'Chest';
        if (item === Item.Drill) return 'Drill';
        return 'Unknown';
    }
    
    // Draw items on belts (smaller than 1 tile, with label)
    const beltItems = gameState.belt_items();
    if (beltItems && beltItems.length > 0) {
        for (let i = 0; i < beltItems.length; i++) {
            const beltItem = beltItems[i];
            const itemX = beltItem.x * TILE_SIZE;
            const itemY = beltItem.y * TILE_SIZE;
            const item = beltItem.item;
            const quantity = beltItem.quantity;
            
            // Draw item as a smaller square (12x12 pixels, centered in tile)
            const itemSize = 12;
            const offsetX = (TILE_SIZE - itemSize) / 2;
            const offsetY = (TILE_SIZE - itemSize) / 2;
            
            ctx.fillStyle = getItemColor(item);
            ctx.fillRect(itemX + offsetX, itemY + offsetY, itemSize, itemSize);
            
            // Draw a small highlight
            ctx.fillStyle = 'rgba(255, 255, 255, 0.3)';
            ctx.fillRect(itemX + offsetX + 1, itemY + offsetY + 1, itemSize - 2, itemSize - 2);
            
            // Draw label above item with name and quantity
            const itemName = getItemName(item);
            const labelText = `${itemName} x${quantity}`;
            
            // Measure text to size background
            ctx.font = '9px "Fusion Pixel"';
            const textMetrics = ctx.measureText(labelText);
            const textWidth = textMetrics.width;
            const textHeight = 12;
            const padding = 4;
            
            const labelX = itemX + TILE_SIZE / 2 - textWidth / 2 - padding;
            const labelY = itemY - textHeight - padding - 2;
            
            // Draw semi-transparent background for label
            ctx.fillStyle = 'rgba(0, 0, 0, 0.7)';
            ctx.fillRect(labelX, labelY, textWidth + padding * 2, textHeight + padding);
            
            // Draw border
            ctx.strokeStyle = 'rgba(255, 255, 255, 0.5)';
            ctx.lineWidth = 1;
            ctx.strokeRect(labelX, labelY, textWidth + padding * 2, textHeight + padding);
            
            // Draw text
            ctx.fillStyle = 'rgba(255, 255, 255, 0.9)';
            ctx.fillText(labelText, labelX + padding, labelY + textHeight - 2);
        }
    }
    
    // Draw dropped items (smaller than 1 tile, without label)
    const droppedItems = gameState.dropped_items();
    if (droppedItems && droppedItems.length > 0) {
        for (let i = 0; i < droppedItems.length; i++) {
            const droppedItem = droppedItems[i];
            const itemX = droppedItem.x * TILE_SIZE;
            const itemY = droppedItem.y * TILE_SIZE;
            const item = droppedItem.item;
            const quantity = droppedItem.quantity;
            
            // Draw item as a smaller square (12x12 pixels, centered in tile)
            const itemSize = 12;
            const offsetX = (TILE_SIZE - itemSize) / 2;
            const offsetY = (TILE_SIZE - itemSize) / 2;
            
            ctx.fillStyle = getItemColor(item);
            ctx.fillRect(itemX + offsetX, itemY + offsetY, itemSize, itemSize);
            
            // Draw a small highlight
            ctx.fillStyle = 'rgba(255, 255, 255, 0.3)';
            ctx.fillRect(itemX + offsetX + 1, itemY + offsetY + 1, itemSize - 2, itemSize - 2);
        }
    }
    
    // Draw cursor highlight (yellow outline on the grid box)
    // This is drawn in world coordinates (after translate), so use world tile coordinates
    if (cursorTileX !== null && cursorTileY !== null) {
        const cursorWorldX = cursorTileX * TILE_SIZE;
        const cursorWorldY = cursorTileY * TILE_SIZE;
        
        ctx.strokeStyle = 'rgba(255, 255, 0, 0.6)'; // Light opacity yellow
        ctx.lineWidth = 2;
        ctx.strokeRect(cursorWorldX, cursorWorldY, TILE_SIZE, TILE_SIZE);
    }
    
    // Draw player as 1x1 tile with direction indicator
    // Draw player body (1x1 red square)
    ctx.fillStyle = '#dc2626'; // Red player
    ctx.fillRect(playerPixelX, playerPixelY, TILE_SIZE, TILE_SIZE);
    
    // Draw direction indicator (small black pixel on the edge)
    const direction = gameState.player_direction_value();
    const indicatorSize = 4; // Small pixel size
    let indicatorX = playerPixelX;
    let indicatorY = playerPixelY;
    
    if (direction === 0) { // North - top edge
        indicatorX = playerPixelX + TILE_SIZE / 2 - indicatorSize / 2;
        indicatorY = playerPixelY + 2;
    } else if (direction === 1) { // South - bottom edge
        indicatorX = playerPixelX + TILE_SIZE / 2 - indicatorSize / 2;
        indicatorY = playerPixelY + TILE_SIZE - indicatorSize - 2;
    } else if (direction === 2) { // East - right edge
        indicatorX = playerPixelX + TILE_SIZE - indicatorSize - 2;
        indicatorY = playerPixelY + TILE_SIZE / 2 - indicatorSize / 2;
    } else if (direction === 3) { // West - left edge
        indicatorX = playerPixelX + 2;
        indicatorY = playerPixelY + TILE_SIZE / 2 - indicatorSize / 2;
    }
    
    ctx.fillStyle = '#000000'; // Black direction indicator
    ctx.fillRect(indicatorX, indicatorY, indicatorSize, indicatorSize);
    
    // Restore context
    ctx.restore();
    
    // Draw inventory in top right
    drawInventory();
    
    // Draw console on the right side
    drawConsole();
    
    // Draw help box below viewport
    drawHelpBox();
}

function drawInventory(): void {
    if (!gameState || !ctx) return;
    
    // Get selected item
    const selectedItem = gameState.get_selected_item();
    const availableItems = gameState.get_available_items();
    
    // Helper to check if an item type matches the selected item
    const isItemSelected = (itemType: Item): boolean => {
        if (!selectedItem) return false;
        return selectedItem === itemType;
    };
    
    // Helper to get item count
    const getItemCount = (item: Item): number => {
        if (!gameState) return 0;
        switch (item) {
            case Item.IronOre: return gameState.iron_ore_count;
            case Item.Copper: return gameState.copper_count();
            case Item.Stone: return gameState.stone_count();
            case Item.Coal: return gameState.coal_count();
            case Item.Furnace: return gameState.furnace_count();
            case Item.IronPlate: return gameState.iron_plate_count();
            case Item.Belt: return gameState.belt_count();
            case Item.CopperPlate: return gameState.copper_plate_count();
            case Item.Arm: return gameState.arm_count();
            case Item.Chest: return gameState.chest_count();
            case Item.Drill: return gameState.drill_count();
            default: return 0;
        }
    };
    
    // Helper to get item name
    const getItemDisplayName = (item: Item): string => {
        switch (item) {
            case Item.IronOre: return 'Iron Ore';
            case Item.Copper: return 'Copper';
            case Item.Stone: return 'Stone';
            case Item.Coal: return 'Coal';
            case Item.Furnace: return 'Furnace';
            case Item.IronPlate: return 'Iron Plate';
            case Item.Belt: return 'Belt';
            case Item.CopperPlate: return 'Copper Plate';
            case Item.Arm: return 'Arm';
            case Item.Chest: return 'Chest';
            case Item.Drill: return 'Drill';
            default: return 'Unknown';
        }
    };
    
    // Count items with count > 0
    let itemCount = 0;
    for (const item of availableItems) {
        if (getItemCount(item) > 0) {
            itemCount++;
        }
    }
    
    // Calculate dynamic inventory height
    const titleHeight = 30;
    const itemHeight = 20;
    const padding = 20;
    const minHeight = 60;
    const inventoryHeight = Math.max(minHeight, titleHeight + (itemCount * itemHeight) + padding);
    
    // Draw inventory background
    const inventoryX = VIEWPORT_WIDTH - 150;
    const inventoryY = 10;
    const inventoryWidth = 140;
    
    ctx.fillStyle = 'rgba(0, 0, 0, 0.7)'; // Semi-transparent black background
    ctx.fillRect(inventoryX, inventoryY, inventoryWidth, inventoryHeight);
    
    // Draw border
    ctx.strokeStyle = '#ffffff';
    ctx.lineWidth = 2;
    ctx.strokeRect(inventoryX, inventoryY, inventoryWidth, inventoryHeight);
    
    // Draw inventory text
    ctx.fillStyle = '#ffffff';
    ctx.font = 'bold 16px "Fusion Pixel"';
    ctx.fillText('Inventory', inventoryX + 10, inventoryY + 25);
    
    // Draw items in the same order as availableItems (which is now sorted consistently)
    ctx.font = '14px "Fusion Pixel"';
    let yOffset = 45;
    
    for (const item of availableItems) {
        const count = getItemCount(item);
        if (count > 0) {
            const isSelected = isItemSelected(item);
            // Draw yellow highlight background first
            if (isSelected) {
                ctx.fillStyle = '#ffff00'; // Yellow highlight
                ctx.fillRect(inventoryX + 5, inventoryY + yOffset - 12, inventoryWidth - 10, 18);
            }
            // Then draw text on top
            ctx.fillStyle = isSelected ? '#000000' : '#ffffff';
            ctx.fillText(`${getItemDisplayName(item)}: ${count}`, inventoryX + 10, inventoryY + yOffset);
            yOffset += 20;
        }
    }
}

function drawConsole(): void {
    if (!gameState || !ctx) return;
    
    const consoleX = VIEWPORT_WIDTH + 10;
    const consoleY = 10;
    const consoleWidth = CONSOLE_WIDTH - 20;
    const consoleHeight = VIEWPORT_HEIGHT - 20;
    
    // Draw console background
    ctx.fillStyle = 'rgba(0, 0, 0, 0.8)';
    ctx.fillRect(consoleX, consoleY, consoleWidth, consoleHeight);
    
    // Draw border
    ctx.strokeStyle = '#00ff00';
    ctx.lineWidth = 2;
    ctx.strokeRect(consoleX, consoleY, consoleWidth, consoleHeight);
    
    // Draw console title
    ctx.fillStyle = '#00ff00';
    ctx.font = 'bold 16px "Fusion Pixel"';
    ctx.fillText('Console', consoleX + 10, consoleY + 25);
    
    // Draw console messages
    const messages = gameState.get_console_messages();
    ctx.fillStyle = '#00ff00';
    ctx.font = '12px "Fusion Pixel"';
    
    const maxMessages = Math.floor((consoleHeight - 40) / 18); // 18px per line
    const startIndex = Math.max(0, messages.length - maxMessages);
    
    let yPos = consoleY + 45;
    for (let i = startIndex; i < messages.length; i++) {
        const message = messages[i];
        // Wrap long messages
        const maxChars = Math.floor(consoleWidth / 7); // Approximate chars per line
        if (message.length > maxChars) {
            const lines: string[] = [];
            for (let j = 0; j < message.length; j += maxChars) {
                lines.push(message.substring(j, j + maxChars));
            }
            for (const line of lines) {
                ctx.fillText(line, consoleX + 10, yPos);
                yPos += 18;
            }
        } else {
            ctx.fillText(message, consoleX + 10, yPos);
            yPos += 18;
        }
        
        if (yPos > consoleY + consoleHeight - 10) {
            break;
        }
    }
}

function drawHelpBox(): void {
    if (!ctx) return;
    
    const helpBoxX = 10;
    const helpBoxY = VIEWPORT_HEIGHT + 10;
    const helpBoxWidth = VIEWPORT_WIDTH + CONSOLE_WIDTH - 20;
    const helpBoxHeight = HELP_BOX_HEIGHT - 20;
    
    // Draw help box background
    ctx.fillStyle = 'rgba(0, 0, 0, 0.8)';
    ctx.fillRect(helpBoxX, helpBoxY, helpBoxWidth, helpBoxHeight);
    
    // Draw border
    ctx.strokeStyle = '#ffffff';
    ctx.lineWidth = 2;
    ctx.strokeRect(helpBoxX, helpBoxY, helpBoxWidth, helpBoxHeight);
    
    // Draw title
    ctx.fillStyle = '#ffffff';
    ctx.font = 'bold 14px "Fusion Pixel"';
    ctx.fillText('Controls', helpBoxX + 10, helpBoxY + 20);
    
    // Draw key bindings in columns
    ctx.fillStyle = '#ffffff';
    ctx.font = '11px "Fusion Pixel"';
    
    const keyBindings = [
        // Movement
        ['W/A/S/D', 'Move'],
        ['Space', 'Place item / Add to container'],
        ['Delete', 'Pick up placeable'],
        // Mining & Resources
        ['M', 'Mine resources'],
        ['H', 'Pick up items / Harvest furnace'],
        ['J', 'Drop selected item'],
        // Crafting
        ['F', 'Craft Furnace'],
        ['B', 'Craft Belt'],
        ['P', 'Craft Arm'],
        ['C', 'Craft Chest'],
        ['T', 'Craft Drill'],
        // Inventory
        ['[ / ]', 'Cycle inventory selection'],
        // Interactions
        ['R', 'Rotate belt/arm'],
    ];
    
    const columnWidth = helpBoxWidth / 3;
    const lineHeight = 16;
    const startX = helpBoxX + 15;
    const startY = helpBoxY + 40;
    
    let currentColumn = 0;
    let currentY = startY;
    
    for (const [key, action] of keyBindings) {
        const x = startX + (currentColumn * columnWidth);
        
        // Draw key in bold/yellow
        ctx.fillStyle = '#ffff00';
        ctx.font = 'bold 11px "Fusion Pixel"';
        ctx.fillText(key, x, currentY);
        
        // Draw action
        const keyWidth = ctx.measureText(key).width;
        ctx.fillStyle = '#ffffff';
        ctx.font = '11px "Fusion Pixel"';
        ctx.fillText(` - ${action}`, x + keyWidth + 5, currentY);
        
        currentY += lineHeight;
        
        // Move to next column if we've filled this one
        if (currentY > helpBoxY + helpBoxHeight - 10) {
            currentColumn++;
            currentY = startY;
        }
    }
}

run().catch(console.error);

