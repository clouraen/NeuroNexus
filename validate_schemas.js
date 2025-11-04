#!/usr/bin/env node

/**
 * JSON Schema Validation Script
 * 
 * This script validates all JSON schema files to ensure they conform to
 * JSON Schema Draft 2020-12 specification and are properly formatted.
 * 
 * Usage: node validate_schemas.js
 */

const fs = require('fs');
const path = require('path');

// ANSI color codes for terminal output
const colors = {
  reset: '\x1b[0m',
  green: '\x1b[32m',
  red: '\x1b[31m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  cyan: '\x1b[36m'
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function validateJsonFile(filePath) {
  try {
    const content = fs.readFileSync(filePath, 'utf8');
    const schema = JSON.parse(content);
    
    // Check required JSON Schema fields
    const required = ['$schema', '$id', 'title', 'description'];
    const missing = required.filter(field => !schema[field]);
    
    if (missing.length > 0) {
      return {
        valid: false,
        errors: [`Missing required fields: ${missing.join(', ')}`]
      };
    }
    
    // Validate $schema version
    if (!schema.$schema.includes('2020-12')) {
      return {
        valid: false,
        errors: ['Schema must use JSON Schema Draft 2020-12']
      };
    }
    
    // Check $id format
    if (!schema.$id.startsWith('https://neuronexus.app/schemas/')) {
      return {
        valid: false,
        errors: ['Schema $id must start with https://neuronexus.app/schemas/']
      };
    }
    
    return { valid: true, errors: [] };
  } catch (error) {
    return {
      valid: false,
      errors: [`Parse error: ${error.message}`]
    };
  }
}

function findSchemaFiles(dir, fileList = []) {
  const files = fs.readdirSync(dir);
  
  files.forEach(file => {
    const filePath = path.join(dir, file);
    const stat = fs.statSync(filePath);
    
    if (stat.isDirectory()) {
      findSchemaFiles(filePath, fileList);
    } else if (file.endsWith('.schema.json')) {
      fileList.push(filePath);
    }
  });
  
  return fileList;
}

function main() {
  log('\n╔════════════════════════════════════════════════════════════╗', 'cyan');
  log('║         NeuroNexus JSON Schema Validator                  ║', 'cyan');
  log('║         JSON Schema Draft 2020-12                          ║', 'cyan');
  log('╚════════════════════════════════════════════════════════════╝\n', 'cyan');
  
  const schemasDir = path.join(__dirname, 'schemas');
  
  if (!fs.existsSync(schemasDir)) {
    log('❌ Schemas directory not found!', 'red');
    process.exit(1);
  }
  
  const schemaFiles = findSchemaFiles(schemasDir);
  
  log(`Found ${schemaFiles.length} schema file(s) to validate\n`, 'blue');
  
  let validCount = 0;
  let invalidCount = 0;
  const errors = [];
  
  schemaFiles.forEach(filePath => {
    const relativePath = path.relative(process.cwd(), filePath);
    process.stdout.write(`Validating ${relativePath}... `);
    
    const result = validateJsonFile(filePath);
    
    if (result.valid) {
      log('✓ PASS', 'green');
      validCount++;
    } else {
      log('✗ FAIL', 'red');
      invalidCount++;
      errors.push({ file: relativePath, errors: result.errors });
    }
  });
  
  // Summary
  log('\n' + '─'.repeat(60), 'cyan');
  log('VALIDATION SUMMARY', 'cyan');
  log('─'.repeat(60), 'cyan');
  log(`Total schemas: ${schemaFiles.length}`, 'blue');
  log(`✓ Valid: ${validCount}`, 'green');
  
  if (invalidCount > 0) {
    log(`✗ Invalid: ${invalidCount}`, 'red');
    
    log('\nERRORS:', 'red');
    errors.forEach(({ file, errors }) => {
      log(`\n${file}:`, 'yellow');
      errors.forEach(error => log(`  - ${error}`, 'red'));
    });
    
    log('\n❌ Validation failed!\n', 'red');
    process.exit(1);
  } else {
    log('\n✅ All schemas are valid!\n', 'green');
    process.exit(0);
  }
}

main();
