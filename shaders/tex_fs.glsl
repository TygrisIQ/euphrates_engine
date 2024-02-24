#version 440 core

out vec4 Fragcolor;
in vec2 texcoord;

uniform float fex;
uniform sampler2D texture1;
uniform sampler2D texture2;
void main(){
  Fragcolor = mix(texture(texture2, texcoord), texture(texture1, texcoord), fex) * vec4(1.0f, 1.0f,1.0f,1.0f);
}
