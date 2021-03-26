import mouseSampleFs from './mouseSampleFs';
import shaderToyFs from './shaderToyFs';
import simpleRedFs from './simpleRedFs';

export {mouseSampleFs, shaderToyFs, simpleRedFs};

export interface ShaderCode {
	name: string;
	code: string;
}

const shaders: Array<ShaderCode> = [
	{
		name: 'Simple Red',
		code: simpleRedFs
	},
	{
		name: 'Mouse Example',
		code: mouseSampleFs
	},
	{
		name: 'Shader Toy',
		code: shaderToyFs
	},
];

export default shaders;
