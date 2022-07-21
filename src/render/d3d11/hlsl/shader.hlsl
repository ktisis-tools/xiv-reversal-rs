// VS Output / PS Input

struct VS_OUT
{
	float4 pos : SV_POSITION;
	float4 color : COLOR;
};

// PS Output

struct PS_OUT {
	float4 color : SV_Target;
};

// Vertex Shader

VS_OUT VS( float4 pos : POSITION, float4 color : COLOR )
{
	VS_OUT output;
	output.pos = pos;
	output.color = color;
	return output;
}

// Pixel Shader

PS_OUT PS(VS_OUT input)
{
	PS_OUT output = (PS_OUT)0;
	output.color = input.color;
	return output;
}