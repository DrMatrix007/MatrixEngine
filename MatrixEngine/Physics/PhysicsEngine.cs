using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.GameObjects.Components.TilemapComponents;
using MatrixEngine.MathM;
using MatrixEngine.System;
using SFML.Graphics;
using SFML.System;
using System;
using System.Collections.Generic;
namespace MatrixEngine.Physics {
    public class PhysicsEngine {

        private List<RigidBodyComponent> dynamicRigidBodies;
        private List<ColliderComponent> colliders;


        public System.App app
        {
            get;
            private set;
        }

        public PhysicsEngine(System.App app) {
            this.app = app;
            dynamicRigidBodies = new List<RigidBodyComponent>();
            colliders = new List<ColliderComponent>();
        }

        public void AddRigidbodyToFrame(RigidBodyComponent rigidBodyComponent) {

            dynamicRigidBodies.Add(rigidBodyComponent);


        }
        public void AddColliderToFrame(ColliderComponent rect) {
            colliders.Add(rect);
        }

        public void Update() {


            foreach (var item in dynamicRigidBodies) {
                if (!item.isStatic) {

                    //var multiplier = 1 - item.velocityDrag * app.deltaTime;
                    //multiplier *= app.deltaTime;
                    //if (1 - multiplier <= 0)
                    //multiplier = 0;



                    item.velocity += (item.gravity*app.deltaTime);


                    item.velocity.Log();

                    item.position += (item.velocity * app.deltaTime);
                    //item.velocity -= new Vector2f(item.velocity.X>0?item.velocityDrag:-item.velocityDrag, item.velocity.Y > 0 ? item.velocityDrag : -item.velocityDrag)*app.deltaTime;
                    //item.velocity += -1 * item.velocity.Normalize() * item.velocity.Length()*item.velocityDrag;
                    //Utils.Log(item.velocity);
                    
                    
                    var fric = (item.velocity.Length() < 1 ? item.velocity : new Vector2f(item.velocity.X, item.velocity.Y).Normalize()) * (-1) * item.velocityDrag;



                    item.velocity += fric;

                }
            }

            //work




            var static_list = colliders.ToArray();
            var non_static_list = dynamicRigidBodies.ToArray();


            foreach (var @static in static_list) {

                if (@static.colliderType == ColliderComponent.ColliderType.None) {
                    continue;
                }

                foreach (var nonstatic in non_static_list) {

                    if (nonstatic.colliderComponent.colliderType == ColliderComponent.ColliderType.None) {
                        continue;
                    }

                    if (nonstatic.colliderComponent.colliderType == ColliderComponent.ColliderType.Rect) {

                        if (@static.colliderType == ColliderComponent.ColliderType.Rect) {
                            HandleRectToRect(nonstatic, @static.rect);
                        }
                        if (@static.colliderType == ColliderComponent.ColliderType.Tilemap) {
                            HandleRectToTilemap(nonstatic, @static);
                        }


                    }

                }
            }


            dynamicRigidBodies.Clear();
            colliders.Clear();

        }

        private void HandleRectToTilemap(RigidBodyComponent nonstatic, ColliderComponent @static) {
            var tilemap = @static.GetComponent<TilemapComponent>();
            if (tilemap == null) {
                return;
            }

            var nonstatic_rect = nonstatic.colliderComponent.rect;
            var tile_scale = tilemap.transform.scale;

            var list_rects = new List<Rect>();

            var pos = new Vector2f(0, 0);

            for (float x = -tile_scale.X*2; x < nonstatic_rect.width + tile_scale.X*2; x += tile_scale.X) {
                for (float y = -tile_scale.Y*2; y < nonstatic_rect.height + tile_scale.Y*2; y += tile_scale.Y) {
                    pos = new Vector2f(x, y) + tilemap.position;
                    if (tilemap.GetTileFromWorldPos(pos + nonstatic.position) != null) {
                        list_rects.Add(new Rect((pos + (Vector2f)(Vector2i)nonstatic.position.Round(10)), tile_scale));


                    }
                }
            }
            foreach (var item in list_rects) {
                //item.position.Log();
                HandleRectToRect(nonstatic, item);

            }


        }

        void HandleRectToRect(RigidBodyComponent nonstatic, Rect @static) {
            var result = nonstatic.gameObject.transform.fullRect.GetCollidingFixFromB(@static);


            if (result.axis == Physics.CollidingAxis.None) {
                return;
            }

            var vel = nonstatic.velocity;


            if (result.axis == Physics.CollidingAxis.X) {
                var pos = nonstatic.position;
                pos.X -= result.fixValue;

                nonstatic.position = pos;
                vel.X = 0;


            } else if (result.axis == Physics.CollidingAxis.Y) {
                var pos = nonstatic.position;
                pos.Y -= result.fixValue;
                nonstatic.position = pos;
                
                vel.Y = 0;


            }
            nonstatic.velocity = vel;
        }

    }
}
