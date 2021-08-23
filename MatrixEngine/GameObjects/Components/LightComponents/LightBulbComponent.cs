using System;
using MatrixEngine.Renderers;

namespace MatrixEngine.GameObjects.Components.LightComponents {
    public class LightBulbComponent : LightComponent {

        public float intensity;
        
        
        public LightBulbComponent(float intensity) {
            this.intensity = intensity;
        }


        internal override LightRenderer.LightType lightType { get; } = LightRenderer.LightType.Bulb;
    }
}