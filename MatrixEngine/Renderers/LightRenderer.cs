using System;
using System.Collections.Generic;
using System.Linq;
using MatrixEngine.GameObjects.Components.LightComponents;
using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.StateManagment;
using MatrixEngine.System;
using MatrixEngine.System.Math;
using SFML.Graphics;
using SFML.System;
using System.Threading;
using System.Threading.Tasks;

namespace MatrixEngine.Renderers {
    public class LightRenderer : Renderer {
        internal enum LightType {
            Bulb,
            Sun,
        }

        
        private const float angleStep = 0.001f;

        private List<LightComponent> lightComponents = new List<LightComponent>();
        private List<LightBlockerComponent> lightBlockerComponents = new List<LightBlockerComponent>();

        public override void Render() {
            var lines = new List<Line>();

            foreach (var lightBlockerComponent in lightBlockerComponents) {
                if (lightBlockerComponent.colliderComponent.colliderType == ColliderComponent.ColliderType.Rect) {
                    var blockR = lightBlockerComponent.transform.fullRect;
                    var ls = blockR.ToLines().ToList();
                    lines.AddRange(ls);
                }
            }

            foreach (var lightComponent in lightComponents) {
                var rect = lightComponent.transform.fullRect;
                switch (lightComponent.lightType) {
                    case LightType.Bulb:
                        var c = lightComponent as LightBulbComponent;
                        if (c == null) {
                            return;
                        }

                        for (var i = angleStep; i < 1f; i += angleStep) {
                            var vertexPos = new Vector2f(MathF.Cos(i * 2 * MathF.PI), MathF.Sin(i * 2 * MathF.PI));
                            var vertexPos2 = new Vector2f(MathF.Cos((i + angleStep) * 2 * MathF.PI),
                                MathF.Sin((i + angleStep) * 2 * MathF.PI));
                            // vertexPos ;
                            var center = rect.center;
                            var vertexline = Line.FromPoints(vertexPos * c.intensity + center, center);


                            var postoCheck = lines.Select(e => {
                                // Console.WriteLine(e);
                                // return new Vector2f();
                                return vertexline.GetCollidingPoint(e);
                            }).ToList();

                            // foreach (var vector2F in postoCheck) {
                            //     Console.Write(vector2F);
                            // }
                            // Console.WriteLine();


                            postoCheck.RemoveAll(e =>
                                float.IsInfinity(e.X) || float.IsInfinity(e.Y));


                            /*foreach (var vector2F in postoCheck) {
                                Console.Write(vector2F);
                            }*/

                            // Console.WriteLine();

                            var intes = float.MaxValue;
                            if (postoCheck.Count != 0) {
                                var f = postoCheck.Aggregate((a, b) => a.Distance(center) > b.Distance(center) ? b : a);
                                intes = (f - center).Length();
                            }

                            intes = MathF.Min(c.intensity, (intes));


                            vertexPos2 *= intes;
                            vertexPos *= intes;

                            var Vertexes = new VertexArray(PrimitiveType.Triangles);
                            foreach (var vertex in new[]
                            {
                                new Vertex(rect.center + vertexPos, new Color(255, 255, 255, 100)),

                                new Vertex(rect.center, new Color(255, 255, 255, 100)),
                                new Vertex(rect.center + vertexPos2, new Color(255, 255, 255, 100)),
                            }) {
                                Vertexes.Append(vertex);
                            }
                            // if (Math.Abs(i - 0.5f) < 0.0001f) {
                            //     
                            // }

                            app.window.Draw(Vertexes);
                        }

                        break;
                    case LightType.Sun:
                        break;
                }
            }

            lightComponents.Clear();
            lightBlockerComponents.Clear();
        }


        public LightRenderer(App app) : base(app) {
        }

        public void AddToLightComponents(LightComponent component) {
            lightComponents.Add(component);
        }

        public void AddToBlockerComponents(LightBlockerComponent lightBlockerComponent) {
            lightBlockerComponents.Add(lightBlockerComponent);
        }
    }
}