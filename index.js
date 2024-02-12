import { createRequire } from 'module';


const require = createRequire(import.meta.url);
const { hl } = require('./treesitter.linux-x64-gnu.node');


export { hl };
