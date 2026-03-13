#!/usr/bin/env node

// Test script for multi-dictionary ancient Chinese queries
const { app, tauri } = require('./dist/index.js');

async function testMultiDictionary() {
  try {
    // Initialize the database
    await app.run();
    
    // Test querying "哀" 
    const results = await tauri.invoke('query_word_multi', { 
      word: '哀', 
      dictType: 'ancient' 
    });
    
    console.log('Found', results.length, 'results for "哀":');
    results.forEach((result, index) => {
      console.log(`${index + 1}. Source: ${result.source}, Word: ${result.word}`);
      console.log(`   Definitions:`, result.definitions);
    });
    
    return results;
  } catch (error) {
    console.error('Test failed:', error);
    throw error;
  }
}

// Run the test
if (require.main === module) {
  testMultiDictionary().then(() => {
    console.log('Test completed successfully!');
  }).catch(console.error);
}