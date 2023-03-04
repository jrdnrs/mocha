#shader vertex

uniform mat4 u_projection;

in vec2  a_position;
in vec2  a_dimensions;
in vec3  a_colour;
in float a_opacity;
in vec3  a_borderColour;
in vec4  a_borderRadii;
in float a_borderWeight;

out vec2  v_centreToPoint;
out vec2  v_halfDimensions;
out vec3  v_colour;
out float v_opacity;
out vec3  v_borderColour;
out vec4  v_borderRadii;
out float v_borderWeight;

vec2 CORNERS[4] = vec2[](
    vec2(0.0, 0.0),
    vec2(1.0, 0.0),
    vec2(0.0, 1.0),
    vec2(1.0, 1.0)
);

void main() {
    // we multiply by `vec2(1.0, -1.0)` to flip Y axis, in the API we assume that Y grows downward
    vec2 current = (a_position + a_dimensions * CORNERS[gl_VertexID]) * vec2(1.0, -1.0);
    vec2 centre = (a_position + a_dimensions * 0.5) * vec2(1.0, -1.0);

    v_centreToPoint = current - centre;
    v_halfDimensions = a_dimensions * 0.5;
    v_colour = a_colour;
    v_opacity = a_opacity;
    v_borderColour = a_borderColour;
    v_borderRadii = a_borderRadii;
    v_borderWeight = a_borderWeight;

    gl_Position = u_projection * vec4(current, 0.0, 1.0);
}

///////////////////////////////////////////////////////////////////////////////////////
#shader fragment

#ifdef GL_ES
    #ifdef GL_FRAGMENT_PRECISION_HIGH
        precision highp float;
    #else
        precision mediump float;
    #endif
#endif

in vec2  v_centreToPoint;
in vec2  v_halfDimensions;
in vec3  v_colour;
in float v_opacity;
in vec3  v_borderColour;
in vec4  v_borderRadii;
in float v_borderWeight;

out vec4 fragColour;

// https://iquilezles.org/articles/distfunctions2d/
float signedDistanceRoundRect(vec2 pos, vec2 size, vec4 radii) {
    radii.xy = (pos.x > 0.0) ? radii.xy : radii.wz;
    radii.x  = (pos.y > 0.0) ? radii.x  : radii.y;
    vec2 q = abs(pos) - size + radii.x;
    return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - radii.x;
}

void main() {
    float outerEdge = signedDistanceRoundRect(v_centreToPoint, v_halfDimensions, v_borderRadii);
    float alpha = 1.0 - smoothstep(0.0, 1.0, outerEdge);

    float innerEdge = outerEdge + v_borderWeight;
    float borderMix = smoothstep(0.0, 0.67, innerEdge);

    vec3 finalColour = mix(v_colour, v_borderColour, borderMix);

    fragColour = vec4(finalColour, alpha * v_opacity);
}