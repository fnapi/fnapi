#!/usr/bin/env node
import * as binding from '../binding.js';
import { argv } from 'node:process';

binding.runCli(argv.slice(1))