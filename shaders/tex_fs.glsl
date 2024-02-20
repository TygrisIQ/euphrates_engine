#version 330 core


in vec4 ocolor;
in vec2 texcoord;
out vec4 Fragcolor;
uniform sampler2D ourtexture;
void main(){
  Fragcolor = texture(ourtexture, texcoord);
}
