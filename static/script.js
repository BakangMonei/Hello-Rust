// Calculator state
let calculator = {
    currentValue: '0',
    previousValue: null,
    operation: null,
    waitingForOperand: false,
    memory: 0,
    history: [],
    currentMode: 'basic'
};

// DOM elements
const mainDisplay = document.getElementById('mainDisplay');
const historyDisplay = document.getElementById('historyDisplay');
const memoryIndicator = document.getElementById('memoryIndicator');
const memoryValue = document.getElementById('memoryValue');
const historyList = document.getElementById('historyList');

// API base URL
const API_BASE = 'http://localhost:3000/api';

// Initialize calculator
document.addEventListener('DOMContentLoaded', function() {
    updateDisplay();
    setupModeSwitching();
    setupKeyboardSupport();
});

// Mode switching
function setupModeSwitching() {
    const modeButtons = document.querySelectorAll('.mode-btn');
    const calculators = document.querySelectorAll('.calculator');
    
    modeButtons.forEach(button => {
        button.addEventListener('click', () => {
            const mode = button.dataset.mode;
            
            // Update active button
            modeButtons.forEach(btn => btn.classList.remove('active'));
            button.classList.add('active');
            
            // Show/hide calculators
            calculators.forEach(calc => calc.classList.add('hidden'));
            document.getElementById(mode + 'Calc').classList.remove('hidden');
            
            calculator.currentMode = mode;
        });
    });
}

// Keyboard support
function setupKeyboardSupport() {
    document.addEventListener('keydown', (event) => {
        const key = event.key;
        
        if (key >= '0' && key <= '9') {
            appendNumber(key);
        } else if (key === '.') {
            appendDecimal();
        } else if (key === '+' || key === '-') {
            setOperator(key);
        } else if (key === '*') {
            setOperator('*');
        } else if (key === '/') {
            setOperator('/');
        } else if (key === 'Enter' || key === '=') {
            calculate();
        } else if (key === 'Escape') {
            clearAll();
        } else if (key === 'Backspace') {
            backspace();
        }
    });
}

// Display functions
function updateDisplay() {
    mainDisplay.textContent = formatNumber(calculator.currentValue);
    updateHistoryDisplay();
    updateMemoryIndicator();
    updateHistoryList();
}

function formatNumber(num) {
    if (num === '') return '0';
    
    const number = parseFloat(num);
    if (isNaN(number)) return 'Error';
    
    // Handle very large or small numbers
    if (Math.abs(number) >= 1e10 || (Math.abs(number) < 1e-10 && number !== 0)) {
        return number.toExponential(6);
    }
    
    // Format with appropriate decimal places
    const maxDecimals = 10;
    const formatted = parseFloat(number.toFixed(maxDecimals)).toString();
    
    // Add thousand separators
    return formatted.replace(/\B(?=(\d{3})+(?!\d))/g, ',');
}

function updateHistoryDisplay() {
    if (calculator.previousValue !== null && calculator.operation) {
        historyDisplay.textContent = `${formatNumber(calculator.previousValue)} ${getOperatorSymbol(calculator.operation)}`;
    } else {
        historyDisplay.textContent = '';
    }
}

function updateMemoryIndicator() {
    if (calculator.memory !== 0) {
        memoryIndicator.textContent = 'M';
        memoryValue.textContent = formatNumber(calculator.memory.toString());
    } else {
        memoryIndicator.textContent = '';
        memoryValue.textContent = '0';
    }
}

function updateHistoryList() {
    historyList.innerHTML = '';
    calculator.history.slice(-10).reverse().forEach(item => {
        const historyItem = document.createElement('div');
        historyItem.className = 'history-item';
        historyItem.innerHTML = `
            <div class="history-expression">${item.expression}</div>
            <div class="history-result">${item.result}</div>
        `;
        historyList.appendChild(historyItem);
    });
}

// Number input functions
function appendNumber(number) {
    if (calculator.waitingForOperand) {
        calculator.currentValue = number;
        calculator.waitingForOperand = false;
    } else {
        if (calculator.currentValue === '0' && number !== '.') {
            calculator.currentValue = number;
        } else {
            calculator.currentValue += number;
        }
    }
    updateDisplay();
}

function appendDecimal() {
    if (calculator.waitingForOperand) {
        calculator.currentValue = '0.';
        calculator.waitingForOperand = false;
    } else if (calculator.currentValue.indexOf('.') === -1) {
        calculator.currentValue += '.';
    }
    updateDisplay();
}

function toggleSign() {
    if (calculator.currentValue !== '0') {
        calculator.currentValue = calculator.currentValue.startsWith('-') 
            ? calculator.currentValue.substring(1) 
            : '-' + calculator.currentValue;
    }
    updateDisplay();
}

function backspace() {
    if (calculator.currentValue.length > 1) {
        calculator.currentValue = calculator.currentValue.slice(0, -1);
    } else {
        calculator.currentValue = '0';
    }
    updateDisplay();
}

// Clear functions
function clearAll() {
    calculator.currentValue = '0';
    calculator.previousValue = null;
    calculator.operation = null;
    calculator.waitingForOperand = false;
    updateDisplay();
}

function clearEntry() {
    calculator.currentValue = '0';
    updateDisplay();
}

function clearHistory() {
    calculator.history = [];
    updateHistoryList();
}

// API call function
async function callCalculatorAPI(operation, a, b, value) {
    try {
        const requestBody = {
            operation: operation,
            a: a,
            b: b,
            value: value
        };

        const response = await fetch(`${API_BASE}/calculate`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(requestBody)
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        return data;
    } catch (error) {
        console.error('API call failed:', error);
        return {
            success: false,
            error: 'Network error: ' + error.message,
            result: 0
        };
    }
}

// Operator functions
async function setOperator(operator) {
    const inputValue = parseFloat(calculator.currentValue);
    
    if (calculator.previousValue === null) {
        calculator.previousValue = inputValue;
    } else if (calculator.operation) {
        const result = await performCalculation(calculator.previousValue, inputValue, calculator.operation);
        if (result.success) {
            calculator.currentValue = result.result.toString();
            calculator.previousValue = result.result;
        } else {
            alert(result.error || 'Calculation failed');
            return;
        }
    }
    
    calculator.waitingForOperand = true;
    calculator.operation = operator;
    updateDisplay();
}

async function calculate() {
    const inputValue = parseFloat(calculator.currentValue);
    
    if (calculator.previousValue === null || calculator.operation === null) {
        return;
    }
    
    const result = await performCalculation(calculator.previousValue, inputValue, calculator.operation);
    
    if (result.success) {
        // Add to history
        addToHistory(result.expression, formatNumber(result.result.toString()));
        
        calculator.currentValue = result.result.toString();
        calculator.previousValue = null;
        calculator.operation = null;
        calculator.waitingForOperand = true;
        updateDisplay();
    } else {
        alert(result.error || 'Calculation failed');
    }
}

async function performCalculation(firstValue, secondValue, operation) {
    const apiOperation = getAPIOperation(operation);
    const response = await callCalculatorAPI(apiOperation, firstValue, secondValue, null);
    return response;
}

function getAPIOperation(operator) {
    switch (operator) {
        case '+': return 'add';
        case '-': return 'subtract';
        case '*': return 'multiply';
        case '/': return 'divide';
        case 'power': return 'power';
        default: return operator;
    }
}

function getOperatorSymbol(operator) {
    switch (operator) {
        case '+': return '+';
        case '-': return '−';
        case '*': return '×';
        case '/': return '÷';
        default: return operator;
    }
}

// Scientific functions
async function calculateFunction(func) {
    const inputValue = parseFloat(calculator.currentValue);
    
    try {
        const response = await callCalculatorAPI(func, null, null, inputValue);
        
        if (response.success) {
            addToHistory(response.expression, formatNumber(response.result.toString()));
            calculator.currentValue = response.result.toString();
            calculator.waitingForOperand = true;
            updateDisplay();
        } else {
            alert(response.error || 'Calculation failed');
        }
    } catch (error) {
        alert('Error: ' + error.message);
    }
}

// Memory functions
function memoryStore() {
    calculator.memory = parseFloat(calculator.currentValue);
    updateMemoryIndicator();
}

function memoryRecall() {
    calculator.currentValue = calculator.memory.toString();
    calculator.waitingForOperand = true;
    updateDisplay();
}

function memoryAdd() {
    calculator.memory += parseFloat(calculator.currentValue);
    updateMemoryIndicator();
}

function memorySubtract() {
    calculator.memory -= parseFloat(calculator.currentValue);
    updateMemoryIndicator();
}

function memoryClear() {
    calculator.memory = 0;
    updateMemoryIndicator();
}

// History functions
function addToHistory(expression, result) {
    calculator.history.push({
        expression: expression,
        result: result,
        timestamp: new Date()
    });
    
    // Keep only last 50 entries
    if (calculator.history.length > 50) {
        calculator.history.shift();
    }
}

// Enhanced power calculation
async function calculatePower() {
    if (calculator.operation === 'power' && calculator.previousValue !== null) {
        const base = calculator.previousValue;
        const exponent = parseFloat(calculator.currentValue);
        
        const response = await callCalculatorAPI('power', base, exponent, null);
        
        if (response.success) {
            addToHistory(response.expression, formatNumber(response.result.toString()));
            calculator.currentValue = response.result.toString();
            calculator.previousValue = null;
            calculator.operation = null;
            calculator.waitingForOperand = true;
            updateDisplay();
        } else {
            alert(response.error || 'Power calculation failed');
        }
    }
}

// Override calculate function to handle power
const originalCalculate = calculate;
async function calculate() {
    if (calculator.operation === 'power') {
        await calculatePower();
    } else {
        await originalCalculate();
    }
}

// Error handling
window.addEventListener('error', function(e) {
    console.error('Calculator error:', e.error);
    calculator.currentValue = 'Error';
    updateDisplay();
});

// Export calculator for debugging
window.calculator = calculator; 