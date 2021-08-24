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
        private List<Line> lines = new List<Line>();
        public override void Render() {
            lines.Clear();

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
                        var Vertexes = new VertexArray(PrimitiveType.Triangles);
                        var fullVertexes = new VertexArray(PrimitiveType.Triangles);

                        var lightsp = c.maxPower / (1 + c.shadowToLightRatio);
                        var lightp = c.maxPower * (1 - 1 / (1 + c.shadowToLightRatio));
                        for (var i = angleStep; i < 1f; i += angleStep) {
                            var vertexPos = new Vector2f(MathF.Cos(i * 2 * MathF.PI), MathF.Sin(i * 2 * MathF.PI));
                            var fullvertexPos = vertexPos * c.intensity;
                            var vertexPos2 = new Vector2f(MathF.Cos((i + angleStep) * 2 * MathF.PI),
                                MathF.Sin((i + angleStep) * 2 * MathF.PI));
                            var fullvertexPos2 = vertexPos2 * c.intensity;


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

                            var intes = c.intensity;
                            if (postoCheck.Count != 0) {
                                var f = postoCheck.Aggregate((a, b) => a.Distance(center) > b.Distance(center) ? b : a);
                                intes = (f - center).Length();
                            }



                            vertexPos2 *= intes;
                            vertexPos *= intes;


                            var p = (byte)MathUtils.LerpToZero(lightsp * 255, intes / c.intensity);


                            foreach (var vertex in new[]
                            {
                                new Vertex(center + vertexPos, new Color(255, 255, 255,p )),

                                new Vertex(center, new Color(255, 255, 255, (byte)(lightsp*255))),
                                new Vertex(center + vertexPos2, new Color(255, 255, 255, p)),
                            }) {
                                Vertexes.Append(vertex);
                            }

                            foreach (var v in new[]
                            {
                                new Vertex(center,new Color(255,255,255,(byte)(lightp*255))),
                                new Vertex(fullvertexPos+center,new Color(255,255,255,0)),
                                new Vertex(fullvertexPos2+center,new Color(255,255,255,0)),
                            }) {
                                fullVertexes.Append(v);
                            }
                            // if (Math.Abs(i - 0.5f) < 0.0001f) {
                            //     
                            // }


                        }
                        app.window.Draw(Vertexes);
                        app.window.Draw(fullVertexes);
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